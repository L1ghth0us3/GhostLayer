mod anticheat;
mod removal;
mod scanner;
mod utils;

use anticheat::get_known_anti_cheats;
use std::io::{self, Read};
use winapi::um::wincon::SetConsoleOutputCP;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    #[cfg(windows)]
    fn check_unicode_output() {
        use std::os::windows::prelude::*;
        //use winapi::um::wincon::SetConsoleOutputCP;
        use winapi::um::consoleapi::GetConsoleOutputCP;
        use winapi::um::winnls::CP_UTF8;

        unsafe {
            let current_cp = GetConsoleOutputCP();
            //SetConsoleOutputCP(CP_UTF8);
            if current_cp != CP_UTF8 {
                println!("UTF-8 not found... Setting ASCII-only mode....");
                const ASCII_ONLY: bool = true;
            }
        }
    }

    //#[cfg(windows)]
    check_unicode_output();

    println!("🕵️‍♂️ GhostLayer {VERSION} - Anti-Cheat Trace Scanner\n");

    let cheats = get_known_anti_cheats();
    let mut any_found = false;
    let mut specific_found = false;

    println!("Scanning services...");
    for ac in &cheats {
        let found_services = scanner::scan_services(ac);

        if found_services {
            removal::print_removal_steps(ac);
            println!();
            any_found = true;
            specific_found = true;
        }
    }

    if !specific_found {
        println!("ℹ️ No offending services found !");
    }

    let mut specific_found = false;

    println!("Scanning File System...");
    for ac in &cheats {
        let found_files = scanner::scan_file_system(ac);

        if found_files {
            removal::print_removal_steps(ac);
            println!();
            any_found = true;
            specific_found = true;
        }
    }

    if !specific_found {
        println!("ℹ️ No offending files found !");
    }

    println!();
    if !any_found {
        println!("✅ No known anti-cheat drivers found on this system.");
    } else {
        println!(
            "🚨 Kernel Level Anticheat Software found!\n🚨 Please follow the removal instructions above to remove it."
        )
    }

    // Pause at the end
    println!("Press Enter to exit...");
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
