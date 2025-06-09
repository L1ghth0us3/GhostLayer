use crate::anticheat::AntiCheat;
use crate::utils::service_exists;
use std::path::Path;

pub fn scan_file_system(ac: &AntiCheat) -> bool {
    //println!("Scanning File System...");
    let mut found = false;
    for path in &ac.file_paths {
        let p = Path::new(path);
        if p.exists() {
            println!();
            println!("🚨 Detected file: {}", ac.name);
            println!("   → File: {}", path);
            found = true;
        }
    }
    found
}

pub fn scan_services(ac: &AntiCheat) -> bool {
    //
    let mut found = false;

    for service in &ac.service_names {
        if service_exists(service) {
            println!();
            println!("🚨 Detected service: {}", service);
            println!("   → {} (Anti-Cheat: {})", service, ac.name);
            found = true;
        }
    }

    found
}
