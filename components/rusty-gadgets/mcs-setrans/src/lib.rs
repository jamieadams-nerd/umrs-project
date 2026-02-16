use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use log::{info, debug, warn};

// Your existing category library from umrs-selinux
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
    /// Sidecar map for the # detail comments (your notes)
    pub details: BTreeMap<SecurityRange, String>,
}

/// The Global Singleton (Initialized with both maps)
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

    /// Returns a list of all markings (Label, Detail) that the given context is authorized to read
    //pub fn list_readable_markings(&self, proc_ctx: &SecurityRange) -> Vec<(String, String)> {
        //self.rules.iter()
            //.filter(|(rule_range, _)| proc_ctx.can_read(rule_range))
            //.map(|(range, label)| {
                //let detail = self.get_detail(range);
                //(label.clone(), detail)
            //})
            //.collect()
    //}
    /// Updated: Returns (Range, Label, Detail)
    pub fn list_readable_markings(&self, proc_ctx: &SecurityRange) -> Vec<(SecurityRange, String, String)> {
        self.rules.iter()
            .filter(|(rule_range, _)| proc_ctx.can_read(rule_range))
            .map(|(range, label)| {
                let detail = self.get_detail(range);
                (range.clone(), label.clone(), detail)
            })
            .collect()
    }

}

// --- STRICT PARSER LOGIC ---
// ===========================================================================
impl SecurityLevel {
    /// Returns true if 'self' dominates 'other'
    /// Math: (Self Sensitivity >= Other Sensitivity) AND (Self Categories are a SUPERSET of Other Categories)
     pub fn dominates(&self, other: &SecurityLevel) -> bool {
        // 1. Sensitivity check: Self must be at least as high as Other
        if self.sensitivity < other.sensitivity {
            return false;
        }

        // 2. Category check: Self must contain EVERY category that Other has
        // We iterate through every category in 'other'
        for cat in other.categories.iter() {
            // Triple-check: Ensure your CategorySet has a 'contains' method
            // If it doesn't, we can use other.categories.iter().all(|c| ...)
            if !self.categories.contains(cat) {
                return false;
            }
        }

        true
    }

}

impl FromStr for SecurityLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (sens_part, cat_part) = if let Some((s_p, c_p)) = s.split_once(':') {
            (s_p, Some(c_p))
        } else {
            (s, None)
        };

        // 1. Strict Sensitivity Check: Alert on 'S'
        if sens_part.starts_with('S') {
            return Err(format!("Syntax Error: Uppercase 'S' is invalid. Did you mean 's{}'?", &sens_part[1..]));
        }
        if !sens_part.starts_with('s') {
            return Err(format!("Syntax Error: Sensitivity must start with 's' (found '{}')", sens_part));
        }

        let sensitivity = sens_part.trim_start_matches('s').parse::<u32>()
            .map_err(|_| format!("Invalid sensitivity format: {}", sens_part))?;

        let mut categories = CategorySet::new();
        let mut last_cat_id: Option<u32> = None;

        if let Some(c_str) = cat_part {
            for part in c_str.split(',') {
                let part = part.trim();
                
                // 2. Strict Category Check: Alert on 'C'
                if part.contains('C') {
                    // Find the number to provide a helpful hint
                    let hint = part.to_lowercase();
                    return Err(format!("Syntax Error: Uppercase 'C' is invalid in '{}'. Use lowercase '{}'.", part, hint));
                }

                let current_id = if let Some((start_s, _)) = part.split_once('.') {
                    start_s.trim_start_matches('c').parse::<u32>().ok()
                } else {
                    part.trim_start_matches('c').parse::<u32>().ok()
                };

                // 3. Style Auditor (Order)
                if let Some(curr) = current_id {
                    if let Some(last) = last_cat_id {
                        if curr < last {
                            debug!("STYLE ALERT: Categories out of order ('{}' follows 'c{}').", part, last);
                        }
                    }
                    last_cat_id = if let Some((_, end_s)) = part.split_once('.') {
                        end_s.trim_start_matches('c').parse::<u32>().ok()
                    } else {
                        Some(curr)
                    };
                }

                // 4. Actual Parse Logic
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

// ===========================================================================
impl SecurityRange {
    /// The "Read" Check: Returns true if the process (self) can read the target (file_range)
    pub fn can_read(&self, file_range: &SecurityRange) -> bool {
        // In most SELinux policies, we compare the "low" levels for simple read access
        self.low.dominates(&file_range.low)
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

// ===========================================================================
/// THE LOADER (First-Match Wins with Strict Error Handling)
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
                    if let Some(old) = translator.rules.get(&range) {
                        warn!("Line {}: IGNORED DUPLICATE! Range '{}' is already '{}'.", line_num, raw_range, old);
                    } else {
                        if !comment.is_empty() {
                            debug!("Line {}: Loaded '{}' -> '{}' | Detail: {}", line_num, raw_range, label, comment);
                        } else {
                            debug!("Line {}: Loaded '{}' -> '{}'", line_num, raw_range, label);
                        }
                        translator.add_rule(range, label.to_string(), comment.to_string());
                    }
                }
                Err(e) => warn!("Line {}: Syntax error on '{}' - {}", line_num, raw_range, e),
            }
        }
    }
    info!("Load complete. {} unique rules in memory.", translator.rules.len());
    Ok(())
}

