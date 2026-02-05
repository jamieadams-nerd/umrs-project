use glob::glob;
use nix::sys::statvfs::statvfs;
use std::collections::HashMap;
use walkdir::WalkDir;

use crate::config::*;
use crate::model::*;

/* ---------- path usage (file | dir | glob) ---------- */

fn path_usage(path_expr: &str) -> Result<Option<u64>, String> {
    let mut total: u64 = 0;
    let mut matched = false;

    for entry in glob(path_expr).map_err(|e| format!("invalid glob {}: {}", path_expr, e))? {
        let p = match entry {
            Ok(p) => p,
            Err(e) => return Err(format!("glob error {}: {}", path_expr, e)),
        };

        matched = true;

        if p.is_file() {
            let meta = p
                .metadata()
                .map_err(|e| format!("metadata error {:?}: {}", p, e))?;
            total += meta.len();
        } else if p.is_dir() {
            for e in WalkDir::new(&p).follow_links(false) {
                let e = e.map_err(|e| format!("walk error {:?}: {}", p, e))?;
                if e.file_type().is_file() {
                    total += e
                        .metadata()
                        .map_err(|e| format!("metadata error {:?}: {}", e.path(), e))?
                        .len();
                }
            }
        }
    }

    if matched {
        Ok(Some(total))
    } else {
        Ok(None)
    }
}

/* ---------- string → enum mapping ---------- */

fn parse_lifecycle(s: &str) -> Result<LifecycleState, String> {
    match s {
        "active" => Ok(LifecycleState::Active),
        "inactive" => Ok(LifecycleState::Inactive),
        "archived" => Ok(LifecycleState::Archived),
        "offline" => Ok(LifecycleState::Offline),
        _ => Err(format!("unknown lifecycle state '{}'", s)),
    }
}

fn parse_class(s: &str) -> Result<LogClass, String> {
    match s {
        "audit" => Ok(LogClass::Audit),
        "system" => Ok(LogClass::System),
        "application" => Ok(LogClass::Application),
        "umrs" => Ok(LogClass::Umrs),
        _ => Err(format!("unknown log class '{}'", s)),
    }
}

/* ---------- filesystem helpers ---------- */

fn filesystem_capacity(path: &str) -> Result<(u64, u64), String> {
    let vfs = statvfs(path).map_err(|e| format!("statvfs failed for {}: {}", path, e))?;

    let block_size = vfs.block_size() as u64;
    let total = (vfs.blocks() as u64) * block_size;
    let free = (vfs.blocks_available() as u64) * block_size;

    Ok((total, free))
}

/* ---------- config → measurement binding ---------- */

pub fn measure_from_config(cfg: &Config) -> Result<Vec<ResourcePool>, String> {
    let mut pools = Vec::new();

    for pool_cfg in &cfg.pool {
        let (total, free) = filesystem_capacity(&pool_cfg.mount_point)?;

        let mut lifecycle_map: HashMap<LifecycleState, Vec<LogConsumer>> = HashMap::new();

        for lc in &pool_cfg.lifecycle {
            let state = parse_lifecycle(&lc.state)?;

            for path_cfg in &lc.paths {
                let class = parse_class(&path_cfg.class)?;

                match path_usage(&path_cfg.path)? {
                    Some(bytes) => {
                        eprintln!("DEBUG: raw bytes {} = {}", path_cfg.path, bytes);
                        lifecycle_map
                            .entry(state.clone())
                            .or_default()
                            .push(LogConsumer {
                                class,
                                bytes_used: bytes,
                            });
                    }
                    None => {
                        eprintln!(
                            "warning: configured path matched nothing, skipping: {}",
                            path_cfg.path
                        );
                    }
                }
            }
        }

        let lifecycles = lifecycle_map
            .into_iter()
            .map(|(state, consumers)| {
                let total_bytes = consumers.iter().map(|c| c.bytes_used).sum();

                LifecycleUsage {
                    state,
                    consumers,
                    total_bytes,
                }
            })
            .collect();

        pools.push(ResourcePool {
            name: pool_cfg.name.clone(),
            mount_point: pool_cfg.mount_point.clone(),
            total_bytes: total,
            free_bytes: free,
            lifecycles,
        });
    }

    Ok(pools)
}
