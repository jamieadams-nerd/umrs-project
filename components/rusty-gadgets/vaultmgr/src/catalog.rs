// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams (a.k.a, Imodium Operator)

use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Catalog {
    #[allow(unused)]
    pub labels: HashMap<String, Label>,
    pub markings: HashMap<String, Marking>,
}

pub fn load_catalog<P: AsRef<Path>>(path: P) -> Result<Catalog, String> {
    let path_ref = path.as_ref();

    let file = File::open(path_ref)
        .map_err(|e| format!("Failed to open {}: {}", path_ref.display(), e))?;

    let reader = BufReader::new(file);

    serde_json::from_reader(reader).map_err(|e| {
        format!("Failed to parse JSON {}: {}", path_ref.display(), e)
    })
}

impl Catalog {
    // ===============
    //  Labels - Lookup a label by key (e.g. "CUI", "GENERAL")
    // ===============
    #[allow(unused)]
    pub fn label(&self, key: &str) -> Option<&Label> {
        self.labels.get(key)
    }

    // Iterate all labels
    #[allow(unused)]
    pub fn iter_labels(&self) -> impl Iterator<Item = (&String, &Label)> {
        self.labels.iter()
    }

    // ===============
    // Markings - Lookup a marking by its JSON key (e.g. "CUI//LEI/JUV").
    // ===============
    pub fn marking(&self, key: &str) -> Option<&Marking> {
        self.markings.get(key)
    }

    // Iterate all markings (key, marking struct).
    #[allow(unused)]
    pub fn iter_markings(&self) -> impl Iterator<Item = (&String, &Marking)> {
        self.markings.iter()
    }

    // Return all direct children of a marking key.
    // Example: "CUI//LEI" → iterable of LEI subcategories
    #[allow(unused)]
    pub fn marking_children<'a>(
        &'a self,
        parent_key: &str,
    ) -> impl Iterator<Item = (&'a String, &'a Marking)> {
        // Extract the last segment of the key
        let parent_segment =
            parent_key.rsplit("//").next().unwrap_or(parent_key);

        self.markings
            .iter()
            .filter(move |(_, m)| m.parent_group == parent_segment)
    }
}

// ===========================================================================

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Label {
    pub name: String,
    pub level: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub handling: String,
}

// ===========================================================================

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Marking {
    pub name: String,
    pub abbrv_name: String,
    pub parent_group: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub handling: String,

    #[serde(default)]
    pub handling_group_id: String,

    #[serde(default)]
    pub other: serde_json::Value,
}

impl Marking {
    #[allow(unused)]
    pub fn has_description(&self) -> bool {
        !self.description.trim().is_empty()
    }

    #[allow(unused)]
    pub fn has_handling(&self) -> bool {
        !self.handling.trim().is_empty()
    }

    #[allow(unused)]
    pub fn has_handling_group(&self) -> bool {
        !self.handling_group_id.trim().is_empty()
    }

    #[allow(unused)]
    pub fn has_other(&self) -> bool {
        match &self.other {
            serde_json::Value::Object(map) => !map.is_empty(),
            serde_json::Value::Null => false,
            _ => true,
        }
    }
}

// ===========================================================================
