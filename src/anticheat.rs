use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AntiCheat {
    pub name: String,
    pub description: String,
    pub driver_files: Vec<String>,
    pub service_names: Vec<String>,
    pub file_paths: Vec<String>,
    pub removal_instructions: Vec<String>,
}

//noinspection ALL
pub fn get_known_anti_cheats() -> Vec<AntiCheat> {
    vec![
        AntiCheat {
            name: "Riot Vanguard".to_string(),
            description: "Kernel-level anti-cheat used by Valorant.".to_string(),
            driver_files: vec!["vgk.sys".to_string()],
            service_names: vec!["vgk".to_string()],
            file_paths: vec!["C:\\Windows\\System32\\drivers\\vgk.sys".to_string()],
            removal_instructions: vec![
                "Uninstall 'Riot Vanguard' via Control Panel.".to_string(),
                "Run `sc delete vgk` in an elevated Command Prompt.".to_string(),
                "Delete C:\\Windows\\System32\\drivers\\vgk.sys".to_string(),
                "Reboot your PC.".to_string(),
            ],
        },
        AntiCheat {
            name: "BattlEye".to_string(),
            description: "Used in PUBG, Rainbow Six: Siege, DayZ, ARK, and more.".to_string(),
            driver_files: vec!["BEDaisy.sys".to_string()],
            service_names: vec!["BEDaisy".to_string()],
            file_paths: vec!["C:\\Windows\\System32\\drivers\\BEDaisy.sys".to_string()],
            removal_instructions: vec![
                "Navigate to the game folder.".to_string(),
                "Run the included Uninstall_BattlEye.bat file if available.".to_string(),
                "Run `sc delete BEDaisy`.".to_string(),
                "Delete BEDaisy.sys manually if it remains.".to_string(),
                "Reboot your PC.".to_string(),
            ],
        },
        AntiCheat {
            name: "Easy Anti-Cheat".to_string(),
            description: "Used by Apex Legends, Fortnite, Rust, and many others.".to_string(),
            driver_files: vec!["EasyAntiCheat.sys".to_string(), "EAC.sys".to_string()],
            service_names: vec!["EasyAntiCheat", "EAC"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            file_paths: vec![
                "C:\\Windows\\System32\\drivers\\EasyAntiCheat.sys".to_string(),
                "C:\\Windows\\System32\\drivers\\EAC.sys".to_string(),
            ],
            removal_instructions: vec![
                "Locate and run EasyAntiCheat_Setup.exe from your game's install folder."
                    .to_string(),
                "Run `sc delete EasyAntiCheat` and/or `sc delete EAC`.".to_string(),
                "Manually delete any remaining .sys files.".to_string(),
                "Reboot your PC.".to_string(),
            ],
        },
        AntiCheat {
            name: "FACEIT Anti-Cheat".to_string(),
            description: "Used in the FACEIT competitive gaming platform.".to_string(),
            driver_files: vec!["faceit.sys".to_string()],
            service_names: vec!["faceit".to_string()],
            file_paths: vec!["C:\\Windows\\System32\\drivers\\faceit.sys".to_string()],
            removal_instructions: vec![
                "Uninstall FACEIT AC from Windows Apps.".to_string(),
                "Run `sc delete faceit`.".to_string(),
                "Reboot your PC.".to_string(),
            ],
        },
        AntiCheat {
            name: "Ricochet".to_string(),
            description: "Used by Call of Duty: Warzone, Modern Warfare II.".to_string(),
            driver_files: vec!["ricochet.sys".to_string()],
            service_names: vec!["ricochet".to_string()],
            file_paths: vec!["C:\\Windows\\System32\\drivers\\ricochet.sys".to_string()],
            removal_instructions: vec![
                "Uninstall the Call of Duty game from Battle.net or Steam.".to_string(),
                "Run `sc delete ricochet`.".to_string(),
                "Delete the ricochet.sys file if it remains.".to_string(),
                "Reboot your PC.".to_string(),
            ],
        },
        AntiCheat {
            name: "XIGNCODE3".to_string(),
            description:
                "Anti-cheat commonly used in Korean and Asian MMOs (e.g., Black Desert Online)."
                    .to_string(),
            driver_files: vec!["xhunter1.sys".to_string()],
            service_names: vec!["xhunter1".to_string()],
            file_paths: vec!["C:\\Windows\\System32\\drivers\\xhunter1.sys".to_string()],
            removal_instructions: vec![
                "Uninstall the game that includes XIGNCODE3.".to_string(),
                "Run `sc delete xhunter1`.".to_string(),
                "Delete xhunter1.sys if it remains.".to_string(),
                "Reboot your PC.".to_string(),
            ],
        },
        AntiCheat {
            name: "MiHoYo Anti-Cheat".to_string(),
            description: "Used in Genshin Impact and Honkai: Star Rail.".to_string(),
            driver_files: vec!["mhyprot2.sys".to_string()],
            service_names: vec!["mhyprot2".to_string()],
            file_paths: vec!["C:\\Windows\\System32\\drivers\\mhyprot2.sys".to_string()],
            removal_instructions: vec![
                "Uninstall Genshin Impact or Honkai: Star Rail.".to_string(),
                "Run `sc delete mhyprot2`.".to_string(),
                "Delete mhyprot2.sys manually.".to_string(),
                "Reboot your PC.".to_string(),
            ],
        }, //,
           //AntiCheat {
           //    name: "Null-Test".to_string(),
           //    description: "Used for testing.".to_string(),
           //    driver_files: vec!["Null.sys".to_string()],
           //    service_names: vec!["null".to_string()],
           //    file_paths: vec!["C:\\Windows\\System32\\drivers\\Null.sys".to_string()],
           //    removal_instructions: vec![
           //        "Only for testing purposes.".to_string(),
           //    ],
           //}
    ]
}
