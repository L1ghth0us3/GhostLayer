mod anticheat;
mod removal;
mod scanner;
mod utils;

use anticheat::get_known_anti_cheats;
use std::io::{self, Read};

fn main() {
    println!("🕵️‍♂️ GhostLayer - Anti-Cheat Trace Scanner\n");

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

    //for ac in &cheats {
    //    let found_files = scanner::scan_file_system(ac);
    //    let found_services = scanner::scan_services(ac);

    //    if found_files || found_services {
    //        removal::print_removal_steps(ac);
    //        println!();
    //        any_found = true;
    //    }
    //}

    println!();
    if !any_found {
        println!("✅ No known anti-cheat drivers found on this system.");
    } else {
        println!(
            "🚨 Kernel Level Anticheat Software found!\n🚨 Please follow the removal instructions above to remove it."
        )
    }

    // ✅ Pause at the end
    println!("Press Enter to exit...");
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
