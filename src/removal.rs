use crate::anticheat::AntiCheat;
use crate::ui::OutputStyle;

pub fn print_removal_steps(ac: &AntiCheat, output_style: &OutputStyle) {
    println!(
        "{} Removal Instructions for {}:",
        output_style.symbol("clean"),
        ac.name
    );
    for (i, step) in ac.removal_instructions.iter().enumerate() {
        println!("  {}. {}", i + 1, step);
    }
}
