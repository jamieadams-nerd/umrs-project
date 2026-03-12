use std::fs;
use std::path::Path;

use umrs_core::console::*;

pub fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<(), String> {
    let path_ref = path.as_ref();

    // If the directory already exists, then we stop. WE do not destory
    // any existing structure.
    //
    // If it doesn exist, then we create the directory of course.
    //
    if path_ref.exists() {
        if path_ref.is_dir() {
            //let msg = format!("Directory already exists: {}", path_ref.display());
            console_info!("Directory already exists: {}", path_ref.display());
            //console_info!("{}" msg);

            return Err(format!(
                "Directory already exists: {}",
                path_ref.display()
            ));
        } else {
            return Err(format!(
                "Path exists but is not a directory: {}",
                path_ref.display()
            ));
        }
    }

    fs::create_dir_all(path_ref).map_err(|e| {
        format!("Failed to create directory {}: {}", path_ref.display(), e)
    })?;

    println!("[CREATE] Directory created: {}", path_ref.display());
    Ok(())
}
