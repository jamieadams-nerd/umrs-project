use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use log::{trace, debug, info, warn, error};

// Pulling in your existing category library
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


#[derive(Debug, Clone)]
pub struct Translation {
    pub label: String,
    pub detail: String, // Store the comment here
}

pub struct Translator {
    /// Mapping the full Range (Key) to the Human Label (Value). (Existing)
    pub rules: BTreeMap<SecurityRange, String>,
    
    /// NEW: Mapping the same Range (Key) to the Comment/Detail string.
    /// This is a "Sidecar" map—if no comment exists, we just don't store it here.
    pub details: BTreeMap<SecurityRange, String>,
}


/// The Global Singleton (Lazy Import)
pub static GLOBAL_TRANSLATOR: Lazy<RwLock<Translator>> = Lazy::new(|| {
    RwLock::new(Translator { rules: BTreeMap::new() })
});

/// Translator
impl Translator {
    pub fn new() -> Self {
        Self { 
            rules: BTreeMap::new(),
            details: BTreeMap::new(),
        }
    }

    /// UPDATED: Now accepts an optional detail string.
    pub fn add_rule(&mut self, range: SecurityRange, label: String, detail: String) {
        self.rules.insert(range.clone(), label);
        if !detail.is_empty() {
            self.details.insert(range, detail);
        }
    }

    /// FORWARD: Existing lookup stays the same!
    pub fn lookup(&self, range: &SecurityRange) -> Option<String> {
        self.rules.get(range).cloned()
    }

    /// NEW: Specifically for your "Triple Check" detail
    pub fn get_detail(&self, range: &SecurityRange) -> String {
        self.details.get(range).cloned().unwrap_or_default()
    }

    /// REVERSE: Query by marking string -> Returns (Kernel String, Detail)
    pub fn lookup_by_marking(&self, marking: &str) -> Vec<(String, String)> {
        let marking = marking.trim();
        self.rules.iter()
            .filter(|(_, label)| label.as_str() == marking)
            .map(|(range, label)| {
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

        let sensitivity = sens_part
            .trim_start_matches('s')
            .trim()
            .parse::<u32>()
            .map_err(|_| format!("Invalid sensitivity format: {}", sens_part))?;

        let mut categories = CategorySet::new();
        if let Some(c_str) = cat_part {
            // Split by comma (e.g., "c0,c1.c3")
            for part in c_str.split(',') {
                let part = part.trim();
                if let Some((start_str, end_str)) = part.split_once('.') {
                    // Handle range expansion: c0.c1023
                    let start = start_str.trim_start_matches('c').parse::<u32>().map_err(|_| "Bad category range start")?;
                    let end = end_str.trim_start_matches('c').parse::<u32>().map_err(|_| "Bad category range end")?;
                    for i in start..=end {
                        let cat_name = format!("c{}", i);
                        categories.insert(Category::from_str(&cat_name).map_err(|e| format!("{:?}", e))?); 
                    }
                } else if !part.is_empty() {
                    // Handle single category: c0
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
        
        // Check if it's a range (contains '-') or a single level
        if let Some((low_str, high_str)) = s.split_once('-') {
            // It's a range: s0-s1
            Ok(SecurityRange {
                low: SecurityLevel::from_str(low_str.trim())?,
                high: SecurityLevel::from_str(high_str.trim())?,
            })
        } else {
            // It's a single level: s0:c1
            // In SELinux, a single level implies Low == High
            let level = SecurityLevel::from_str(s)?;
            Ok(SecurityRange {
                low: level.clone(),
                high: level,
            })
        }
    }
}


/// THE LOADER: Reads the file and populates the GLOBAL_TRANSLATOR.
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
            
            let (label, inline_comment) = if let Some((l, c)) = rest.split_once('#') {
                (l.trim(), Some(c.trim()))
            } else {
                (rest.trim(), None)
            };

            match SecurityRange::from_str(raw_range) {
                Ok(range) => {
                    // --- THE "FIRST MATCH" LOGIC ---
                    if let Some(existing_label) = translator.rules.get(&range) {
                        // We found a collision! 
                        // Because we want FIRST match, we DISCARD the current line.
                        warn!(
                            "Line {}: IGNORED DUPLICATE! Bitmask for '{}' is already defined as '{}'. Ignoring '{}'.", 
                            line_num, raw_range, existing_label, label
                        );
                    } else {
                        // This is a new, unique bitmask. Add it.
                        if let Some(detail) = inline_comment {
                            debug!("Line {}: Loaded '{}' -> '{}' | Detail: {}", line_num, raw_range, label, detail);
                        } else {
                            debug!("Line {}: Loaded '{}' -> '{}'", line_num, raw_range, label);
                        }
                        translator.add_rule(range, label.to_string());
                    }
                }
                Err(e) => {
                    warn!("Line {}: Failed to parse range '{}' - {}", line_num, raw_range, e);
                }
            }
        }
    }
    
    info!("Load complete. {} unique rules in memory.", translator.rules.len());
    Ok(())
}
