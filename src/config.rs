use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, write, File};
use std::io::Read;

static EXAMPLE_CONFIG: &str = "---
auto_start: true
hotkeys:
  - ALT+RETURN
  - SHIFT+ALT+E
commands:
    - wezterm
    - btm
";

pub fn load_config() -> Config {
    if let Some(mut config_path) = dirs::config_dir() {
        config_path.push("tsttr");
        if !config_path.exists() {
            let _ = create_dir_all(&config_path);
        }

        config_path.push("config.yml");
        if !config_path.exists() {
            let _ = write(&config_path, EXAMPLE_CONFIG);
        }

        let mut config = config::Config::default();
        let _ = config.merge(config::Config::try_from(&Config::default()).unwrap());

        let file_config = config::File::from(config_path).format(config::FileFormat::Yaml);

        if let Ok(config) = config.merge(file_config) {
            return config.clone().try_into().unwrap();
        }
    };

    Config::default()
}

pub fn toggle_autostart() {
    if let Some(mut config_path) = dirs::config_dir() {
        config_path.push("tsttr");
        config_path.push("config.yml");

        if let Ok(mut config) = File::open(&config_path) {
            let mut config_str = String::new();

            let _ = config.read_to_string(&mut config_str);

            let re_line = Regex::new(r"(?m)^(auto_start:)(.*)$").unwrap();
            let updated_config = if let Some(cap) = re_line.captures_iter(&config_str).next() {
                if re_line.captures_len() == 3 {
                    let re_cap =
                        Regex::new(r"(?m)^(y|Y|yes|Yes|YES|true|True|TRUE|on|On|ON)$").unwrap();

                    let enabled = re_cap.find(&cap[2].trim());

                    let updated_config = re_line.replace(&config_str, |caps: &Captures| {
                        format!("{} {}", &caps[1], !enabled.is_some())
                    });

                    Some(updated_config.as_ref().to_owned())
                } else {
                    None
                }
            } else {
                None
            };

            let updated_config = if let Some(updated_config) = updated_config {
                updated_config
            } else {
                format!("{}\n\nauto_start: true", config_str)
            };

            let _ = write(&config_path, updated_config);
        }
    }
}

// #[derive(Debug, Serialize, Deserialize, Clone,)]
// pub struct Keys {
//     pub hotkeys: Vec<Hotkey>,
// }

// #[derive(Debug, Serialize, Deserialize, Clone,)]
// pub struct Hotkey {
//     pub sequence: String,
//     pub command: String
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub hotkey: String,
    pub auto_start: bool,
    pub hotkeys: Vec<String>,
    pub commands: Vec<String>,
    // pub keys: Keys
}

impl Default for Config {
    fn default() -> Self {
        Config {
            hotkey: "CTRL+ALT+W".to_string(),
            auto_start: false,
            hotkeys: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            commands: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            // keys: Keys {hotkeys: vec![]}
        }
    }
}
