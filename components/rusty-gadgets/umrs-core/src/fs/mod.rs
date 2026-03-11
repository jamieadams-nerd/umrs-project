use std::fs;
use std::path::{Path, PathBuf};

//                                                                                                  
// Encrypted mount detection                                                                        
//                                                                                                  
/// Read `/proc/mounts` and return the set of mount-point paths that use a                          
/// known encrypted filesystem type (ecryptfs, gocryptfs, etc.).                                    
fn load_encrypted_mounts() -> HashSet<String> {                                                     
    let mut set = HashSet::new();                                                                   
    let Ok(contents) = std::fs::read_to_string("/proc/mounts") else {                               
        return set;                                                                                 
    };                                                                                              
    for line in contents.lines() {                                                                  
        let mut parts = line.split_whitespace();                                                    
        let _device = parts.next();                                                                 
        let Some(mount_point) = parts.next() else {                                                 
            continue;                                                                               
        };                                                                                          
        let Some(fs_type) = parts.next() else {                                                     
            continue;                                                                               
        };                                                                                          
        if ENCRYPTED_FS_TYPES.contains(&fs_type) {                                                  
            set.insert(mount_point.to_owned());                                                     
        }                                                                                           
    }                                                                                               
    log::debug!("Scanned /proc/mounts for encrypted filesystems.");                                 
                                                                                                    
    set                                                                                             
}


pub fn is_luks_encrypted<P: AsRef<Path>>(mount_point: P) -> bool {
    let target = mount_point.as_ref();

    // 1. Find the device in /proc/self/mounts
    let mounts = fs::read_to_string("/proc/self/mounts").unwrap_or_default();
    let dev_node = mounts.lines().find_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // Index 1 is the mount point, Index 0 is the device
        if parts.len() >= 2 && Path::new(parts[1]) == target {
            Some(parts[0].to_string())
        } else {
            None
        }
    });

    let Some(dev_path) = dev_node else { return false; };

    // 2. Resolve to the kernel name (e.g., /dev/mapper/data -> /dev/dm-0)
    let Ok(real_path) = fs::canonicalize(&dev_path) else { return false; };
    let Some(kernel_name) = real_path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };

    // 3. Check the kernel's device-mapper type in sysfs
    // This is the source of truth for active encryption
    let type_path = format!("/sys/class/block/{}/dm/type", kernel_name);
    if let Ok(dm_type) = fs::read_to_string(type_path) {
        if dm_type.trim() == "crypt" {
            return true;
        }
    }

    // 4. Fallback: Check the UUID for the LUKS prefix
    let uuid_path = format!("/sys/class/block/{}/dm/uuid", kernel_name);
    if let Ok(uuid) = fs::read_to_string(uuid_path) {
        return uuid.trim().starts_with("CRYPT-LUKS");
    }

    false
}

