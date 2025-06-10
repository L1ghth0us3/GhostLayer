use winreg::RegKey;
use winreg::enums::*;

// Check for service via Registry
pub fn service_exists(service_name: &str) -> bool {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let services = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Services");

    match services {
        Ok(services_key) => services_key.open_subkey(service_name).is_ok(),
        Err(_) => false,
    }
}

// Load AC Data from json
use serde_json::Value;
use std::error::Error;
use std::fs;

pub fn load_ac_data() -> Result<Value, Box<dyn Error>> {
    let json_contents = fs::read_to_string("assets/ac_list.json")?;
    let ac_data: Value = serde_json::from_str(&json_contents)?;
    Ok(ac_data)
}
