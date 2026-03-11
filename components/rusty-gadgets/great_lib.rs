use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use log::{info, debug, warn};

// Your existing category library
use umrs_selinux::category::{CategorySet, Category};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecurityLevel {
    pub sensitivity: u32,
    pub categories: CategorySet,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecurityRange {
    pub low: SecurityLevel,
    pub high: SecurityLevel,
}

pub struct Translator {
    pub rules: BTreeMap<SecurityRange, String>,
    /// Sidecar map for the # detail comments
    pub details: BTreeMap<SecurityRange, String>,
}

/// The Global Singleton (Fixed with both maps)
pub static GLOBAL_TRANSLATOR: Lazy<RwLock<Translator>> = Lazy::new(|| {
    RwLock::new(Translator { 
        rules: BTreeMap::new(),
        details: BTreeMap::new(),
    })
});

impl Translator {
    pub fn new() -> Self {
        Self { 
            rules: BTreeMap::new(),
            details: BTreeMap::new(),
        }
    }

    /// Adds a rule and its optional detail sidecar.
    pub fn add_rule(&mut self, range: SecurityRange, label: String, detail: String) {
        if !detail.is_empty() {
            self.details.insert(range.clone(), detail);
        }
        self.rules.insert(range, label);
    }

    /// FORWARD: 's0:c0' -> 'CUI'
    pub fn lookup(&self, range: &SecurityRange) -> Option<String> {
        self.rules.get(range).cloned()
    }

    /// DETAIL: Get the comment for a range
    pub fn get_detail(&self, range: &SecurityRange) -> String {
        self.details.get(range).cloned().unwrap_or_default()
    }

    /// REVERSE: 'CUI' -> ('s0:c0', 'Controlled Unclassified Info')
    pub fn lookup_by_marking(&self, marking: &str) -> Vec<(String, String)> {
        let marking = marking.trim();
        self.rules.iter()
            .filter(|(_, label)| label.as_str() == marking)
            .map(|(range, _label)| {
                let kernel_str = if range.low == range.high {
                    format!("s{}:{}", range.low.sensitivity, range.low.categories)
                } else {
                    format!("s{}:{}-s{}:{}", 
                        range.low.sensitivity, range.low.categories,
                        range.high.sensitivity, range.high.categories)
                };
                let detail = self.get_detail(range);
                (kernel_str, detail)
            })
            .collect()
    }
}

// --- PARSER LOGIC ---
impl FromStr for SecurityLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (sens_part, cat_part) = if let Some((s_p, c_p)) = s.split_once(':') {
            (s_p, Some(c_p))
        } else {
            (s, None)
        };

        let sensitivity = sens_part.trim().trim_start_matches('s').parse::<u32>()
            .map_err(|_| format!("Invalid sensitivity: {}", sens_part))?;

        let mut categories = CategorySet::new();
        let mut last_cat_id: Option<u32> = None; // Track for style check

        if let Some(c_str) = cat_part {
            for part in c_str.split(',') {
                let part = part.trim();
                
                // 1. Determine the current ID for the style check
                let current_id = if let Some((start_s, _)) = part.split_once('.') {
                    start_s.trim_start_matches('c').parse::<u32>().ok()
                } else {
                    part.trim_start_matches('c').parse::<u32>().ok()
                };

                // 2. Perform the Style Check: Alert if out of order
                if let Some(curr) = current_id {
                    if let Some(last) = last_cat_id {
                        if curr < last {
                            // We use debug! so it doesn't clutter normal runs, 
                            // but you can change this to warn! if you want it loud.
                            debug!("STYLE ALERT: Categories out of order ('{}' follows 'c{}').", part, last);
                        }
                    }
                    last_cat_id = Some(curr);
                }

                // 3. Normal Parsing (Keep your existing logic here)
                if let Some((start_str, end_str)) = part.split_once('.') {
                    let start = start_str.trim_start_matches('c').parse::<u32>().map_err(|_| "Bad start")?;
                    let end = end_str.trim_start_matches('c').parse::<u32>().map_err(|_| "Bad end")?;
                    for i in start..=end {
                        let name = format!("c{}", i);
                        categories.insert(Category::from_str(&name).map_err(|e| format!("{:?}", e))?); 
                    }
                } else if !part.is_empty() {
                    categories.insert(Category::from_str(part).map_err(|e| format!("{:?}", e))?);
                }
            }
        }
        Ok(SecurityLevel { sensitivity, categories })
    }
}


impl FromStr for SecurityRange {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Some((low_s, high_s)) = s.split_once('-') {
            Ok(SecurityRange {
                low: SecurityLevel::from_str(low_s)?,
                high: SecurityLevel::from_str(high_s)?,
            })
        } else {
            let level = SecurityLevel::from_str(s)?;
            Ok(SecurityRange { low: level.clone(), high: level })
        }
    }
}

/// THE LOADER (RESTORED: Full debug logging with detail parsing)
pub fn load_setrans_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut translator = GLOBAL_TRANSLATOR.write().map_err(|_| "Lock poisoned")?;

    info!("Loading SELinux translations from: {}", path);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let line_num = index + 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }

        if let Some((raw_range, rest)) = line.split_once('=') {
            let raw_range = raw_range.trim();
            let (label, comment) = if let Some((l, c)) = rest.split_once('#') {
                (l.trim(), c.trim())
            } else {
                (rest.trim(), "")
            };

            match SecurityRange::from_str(raw_range) {
                Ok(range) => {
                    // --- THE "FIRST MATCH" LOGIC WITH FULL LOGGING ---
                    if let Some(existing_label) = translator.rules.get(&range) {
                        warn!(
                            "Line {}: IGNORED DUPLICATE! Range '{}' is already '{}'. Ignoring '{}'.", 
                            line_num, raw_range, existing_label, label
                        );
                    } else {
                        // LOG THE SEXY DEBUG INFO
                        if !comment.is_empty() {
                            debug!("Line {}: Loaded '{}' -> '{}' | Detail: {}", line_num, raw_range, label, comment);
                        } else {
                            debug!("Line {}: Loaded '{}' -> '{}'", line_num, raw_range, label);
                        }
                        translator.add_rule(range, label.to_string(), comment.to_string());
                    }
                }
                Err(e) => warn!("Line {}: Parse error on '{}' - {}", line_num, raw_range, e),
            }
        }
    }
    info!("Load complete. {} unique rules in memory.", translator.rules.len());
    Ok(())
}

