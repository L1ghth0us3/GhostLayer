use crate::anticheat::AntiCheat;

pub fn print_removal_steps(ac: &AntiCheat) {
    println!("🧹 Removal Instructions for {}:", ac.name);
    for (i, step) in ac.removal_instructions.iter().enumerate() {
        println!("  {}. {}", i + 1, step);
    }
}
