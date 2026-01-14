use std::collections::HashMap;
use std::path::Path;

use nix::sys::statvfs::statvfs;
use walkdir::WalkDir;

use crate::config::*;
use crate::model::*;

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
    let vfs = statvfs(path)
        .map_err(|e| format!("statvfs failed for {}: {}", path, e))?;

    let block_size = vfs.block_size() as u64;
    let total = vfs.blocks() * block_size;
    let free = vfs.blocks_available() * block_size;

    Ok((total, free))
}

fn directory_usage(path: &str) -> Result<u64, String> {
    let mut total: u64 = 0;

    for entry in WalkDir::new(path).follow_links(false) {
        let entry = entry.map_err(|e| format!("walk error in {}: {}", path, e))?;
        if entry.file_type().is_file() {
            let meta = entry.metadata()
                .map_err(|e| format!("metadata error {:?}: {}", entry.path(), e))?;
            total += meta.len();
        }
    }

    Ok(total)
}

/* ---------- config → measurement binding ---------- */

pub fn measure_from_config(cfg: &Config) -> Result<Vec<ResourcePool>, String> {
    let mut pools = Vec::new();

    for pool_cfg in &cfg.pool {
        let (total, free) = filesystem_capacity(&pool_cfg.mount_point)?;

        let mut lifecycle_map: HashMap<LifecycleState, Vec<LogConsumer>> =
            HashMap::new();

        for lc in &pool_cfg.lifecycle {
            let state = parse_lifecycle(&lc.state)?;

            for path_cfg in &lc.paths {
                let class = parse_class(&path_cfg.class)?;

                if !Path::new(&path_cfg.path).exists() {
                    return Err(format!("configured path does not exist: {}", path_cfg.path));
                }

                let bytes = directory_usage(&path_cfg.path)?;

                lifecycle_map
                    .entry(state.clone())
                    .or_default()
                    .push(LogConsumer {
                        class,
                        bytes_used: bytes,
                    });
            }
        }

        let lifecycles = lifecycle_map
            .into_iter()
            .map(|(state, consumers)| LifecycleUsage {
                state,
                consumers,
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
