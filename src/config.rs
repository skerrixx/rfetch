use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use colored::Colorize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "defacolor")]
    pub color_ascii: bool,
    #[serde(default = "deficolor")]
    pub color_infotext: String,
    #[serde(default = "defhide")]
    pub hide_info: Vec<String>,
	#[serde(default = "defstyle")]
	pub style: String,
	#[serde(default)]
	pub anonymize: bool,
	#[serde(default)]
	pub ascii_path: Option<String>,
}

fn defacolor() -> bool {
    true
}

fn deficolor() -> String {
    "white".to_string()
}

fn defstyle() -> String {
    "sectioned".to_string()
}

fn defhide() -> Vec<String> {
    Vec::new()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            color_ascii: defacolor(),
            color_infotext: deficolor(),
            hide_info: defhide(),
			style: defstyle(),
			anonymize: false,
			ascii_path: None,
        }
    }
}

fn config_dir() -> PathBuf {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        if !xdg.trim().is_empty() {
            return PathBuf::from(xdg).join("rfetch");
        }
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config").join("rfetch")
}

pub fn config_path() -> PathBuf {
    config_dir().join("conf.jsonc")
}
fn strip_comments(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_string = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;

    while let Some(c) = chars.next() {
        if in_line_comment {
            if c == '\n' {
                in_line_comment = false;
                out.push(c);
            }
            continue;
        }
        if in_block_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_block_comment = false;
            }
            continue;
        }
        if in_string {
            out.push(c);
            if c == '\\' {
                if let Some(n) = chars.next() {
                    out.push(n);
                }
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        match c {
            '"' => {
                in_string = true;
                out.push(c);
            }
            '/' if chars.peek() == Some(&'/') => {
                chars.next();
                in_line_comment = true;
            }
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                in_block_comment = true;
            }
            _ => out.push(c),
        }
    }

    out
}

fn default_config_content() -> String {
    "{\n\t\"color_ascii\": true,\n\t\"color_infotext\": \"white\",\n\t\"hide_info\": [\n\t\t/* \n\t\tuncomment any string below to hide the info about it.\n\t\t*/\n\t\t // \"headers\"\n\t\t // \"packages\"\n\t\t // \"os\"\n\t\t // \"os_age\"\n\t\t // \"kernel\"\n\t\t // \"de/wm\" // (also: \"de_wm\", \"de\", \"wm\")\n\t\t // \"shell\"\n\t\t // \"terminal\" // (also: \"term\")\n\t\t // \"uptime\"\n\t\t // \"boot\"\n\t\t // \"cpu\"\n\t\t // \"gpu\"\n\t\t // \"ram\"\n\t\t // \"swap\"\n\t\t // \"load\" // (also: \"loadavg\", \"load_avg\")\n\t\t // \"processes\" // (also: \"procs\", \"proc\")\n\t\t // \"disk\"\n\t\t // \"battery\" //(only hides it if it's present at all)\n\t],\n\t\"style\": \"sectioned\", // options: sectioned/boxed\n\t\"anonymize\": false, // set true to hide username/hostname for screenshots\n\t\"ascii_path\": null // set to e.g. \"~/.config/rfetch/ascii.txt\" for custom art\n}\n".to_string()
}
pub fn load_config() -> Config {
    let path = config_path();

    if !path.exists() {
        return first_run_setup();
    }

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "rfetch: warning: could not read config {} ({})",
                path.display(),
                e
            );
            eprintln!("rfetch: using default config.");
            return Config::default();
        }
    };

    match serde_json::from_str::<Config>(&strip_comments(&content)) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!(
                "rfetch: warning: failed to parse {} ({})",
                path.display(),
                e
            );
            eprintln!("rfetch: using default config.");
            Config::default()
        }
    }
}

fn first_run_setup() -> Config {
    println!("welcome!");
    println!("{}{}{}{}","it seems it's your ", "first".red(), " time using ", "rfetch!".blue());
    println!(
        "\nwe haven't found a rfetch configuration file found at {}.",
        config_path().display()
    );
    println!("\ncreating a default config - screenshot-ready, no setup needed!");
    println!("tip: edit ~/.config/rfetch/conf.jsonc anytime to customize rfetch.\n");
    let cfg = Config::default();

    if let Some(parent) = config_path().parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            println!(
                "{}{}{}{}",
                "error".red(),
                ": could not create {} ({})",
                parent.display(),
                e
            );
            return cfg;
        }
    }

    match std::fs::write(&config_path(), default_config_content()) {
        Ok(_) => {
            eprintln!();
            eprintln!("your config is created! it's located at {}.", config_path().display());
            eprintln!("you can edit it anytime to customize rfetch.");
        }
        Err(e) => {
            eprintln!();
            eprintln!(
                "rfetch: error: could not write config {} ({})",
                config_path().display(),
                e
            );
        }
    }

    cfg
}

