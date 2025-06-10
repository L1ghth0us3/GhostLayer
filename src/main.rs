mod anticheat;
mod removal;
mod scanner;
mod ui;
mod utils;

use serde_json::Value;
use std::io::{self, Read};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // Initialize the CLI
    let output_style = ui::initialize_cli();

    // load AC Data from JSON
    match utils::load_ac_data() {
        Ok(ac_data) => {
            for (name, entries) in ac_data {
                println!("Anticheat: {}", name);
                for entry in entries {
                    println!("  Game: {}", entry.game);
                    println!("  Kernel: {}", entry.kernel);
                    println!("  OS: {:?}", entry.os);
                    println!("  Notes: {}", entry.notes);
                    println!("  Drivers: {:?}", entry.drivers);
                    println!();
                }
                println!();
            }
        }
        Err(e) => eprintln!("Error loading data: {}", e),
    }

    // Start the Program
    println!(
        "{} GhostLayer {VERSION} - Anti-Cheat Trace Scanner",
        output_style.symbol("scan")
    );

    // Scan for known anti-cheat drivers
    let cheats = anticheat::get_known_anti_cheats();
    let mut any_found = false;
    let mut specific_found = false;

    // Scan services for known anti-cheat drivers
    println!("Scanning services...");
    for ac in &cheats {
        let found_services = scanner::scan_services(ac, &output_style);

        if found_services {
            removal::print_removal_steps(ac, &output_style);
            println!();
            any_found = true;
            specific_found = true;
        }
    }

    if !specific_found {
        println!(
            "{} No offending services found !",
            output_style.symbol("info")
        );
    }

    let mut specific_found = false;

    // Scan file system for known anti-cheat drivers
    println!("Scanning File System...");
    for ac in &cheats {
        let found_files = scanner::scan_file_system(ac, &output_style);

        if found_files {
            removal::print_removal_steps(ac, &output_style);
            println!();
            any_found = true;
            specific_found = true;
        }
    }

    if !specific_found {
        println!("{} No offending files found !", output_style.symbol("info"));
    }

    // Print the results
    println!();
    if !any_found {
        println!(
            "{} No known anti-cheat drivers found on this system.",
            output_style.symbol("ok")
        );
    } else {
        println!(
            "{0} Kernel Level Anticheat Software found!\n\
            {0} Please follow the removal instructions above to remove it.",
            output_style.symbol("found")
        )
    }

    // Pause at the end
    println!("Press Enter to exit...");
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
