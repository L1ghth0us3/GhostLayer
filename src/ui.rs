use crate::ui;

pub enum OutputStyle {
    Emoji,
    Ascii,
}

impl OutputStyle {
    pub fn symbol(&self, kind: &str) -> &str {
        match (self, kind) {
            (Self::Emoji, "scan") => "🔍",
            (Self::Emoji, "warn") => "⚠️",
            (Self::Emoji, "found") => "🚨",
            (Self::Emoji, "info") => "ℹ️",
            (Self::Emoji, "ok") => "✅",
            (Self::Emoji, "clean") => "🧹",
            (Self::Ascii, "scan") => "[*]",
            (Self::Ascii, "warn") => "[!]",
            (Self::Ascii, "found") => "[#]",
            (Self::Ascii, "info") => "(!)",
            (Self::Ascii, "ok") => "[ok]",
            (Self::Ascii, "clean") => "[clean]",
            _ => "",
        }
    }

    pub fn header(&self, label: &str) {
        match self {
            OutputStyle::Emoji => println!("\n✨ {label}"),
            OutputStyle::Ascii => println!("\n=== {label} ==="),
        }
    }
}

#[cfg(windows)]
fn check_unicode_output() -> bool {
    let mut ascii_only: bool = false;
    use std::os::windows::prelude::*;
    //use winapi::um::wincon::SetConsoleOutputCP;
    use winapi::um::consoleapi::GetConsoleOutputCP;
    use winapi::um::winnls::CP_UTF8;

    unsafe {
        let current_cp = GetConsoleOutputCP();
        //SetConsoleOutputCP(CP_UTF8);
        if current_cp != CP_UTF8 {
            println!("UTF-8 not found... Setting ASCII-only mode....");
            ascii_only = true;
        }
    }
    ascii_only
}

pub fn initialize_cli() -> OutputStyle {
    let mut ascii_only = false;

    let ascii_only = check_unicode_output();

    let output_style = if ascii_only {
        OutputStyle::Ascii
    } else {
        OutputStyle::Emoji
    };
    output_style
}
