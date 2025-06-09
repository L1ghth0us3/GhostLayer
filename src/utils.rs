use winreg::RegKey;
use winreg::enums::*;

pub fn service_exists(service_name: &str) -> bool {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let services = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Services");

    match services {
        Ok(services_key) => services_key.open_subkey(service_name).is_ok(),
        Err(_) => false,
    }
}
