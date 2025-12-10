
§use std::path::Path;
§use std::fs;
§use std::io::{self, Read, Write};
§
§use serde::{Deserialize, Serialize};
§
§#[derive(Debug, Serialize, Deserialize)]
§pub struct UmrsState {
§    pub system_metadata: SystemMetadata,
§    // later: dynamic fields, integrity status, etc.
§}
§
§#[derive(Debug, Serialize, Deserialize)]
§pub struct SystemMetadata {
§    pub purpose: Option,
§    pub system_type: Option,
§    pub virtualization: Option,
§}
§
§impl Default for UmrsState {
§    fn default() -> Self {
§        UmrsState {
§            system_metadata: SystemMetadata {
§                purpose: None,
§                system_type: None,
§                virtualization: None,
§            },
§        }
§    }
§}
§
§pub fn load_state(path: &Path) -> io::Result {
§    if !path.exists() {
§        return Ok(UmrsState::default());
§    }
§    let mut file = fs::File::open(path)?;
§    let mut buf = String::new();
§    file.read_to_string(&mut buf)?;
§    let state: UmrsState = serde_json::from_str(&buf)
§        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
§    Ok(state)
§}
§
§pub fn save_state(path: &Path, state: &UmrsState) -> io::Result<()> {
§    let tmp_path = path.with_extension(“json.tmp”);
§    {
§        let mut file = fs::File::create(&tmp_path)?;
§        let data = serde_json::to_string_pretty(state)
§            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
§        file.write_all(data.as_bytes())?;
§        file.sync_all()?;
§    }
§    fs::rename(tmp_path, path)?;
§    Ok(())
§}
