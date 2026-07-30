mod pkgs;
mod basic;
use pkgs::getform;
use colored::Colorize;
use std::env;
use whoami;
use rand;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use std::process::Command;

fn print_usage() {
    eprintln!("Usage: rfetch [--distro <distro> | -d <distro>] [--clear-cache]");
    eprintln!();
    eprintln!("Flags:");
    eprintln!("  -d, --distro <distro>   Override the detected distro art/logo");
    eprintln!("  --clear-cache           Force rebuild of package/drives cache");
    eprintln!();
    eprintln!("Available distros: {}", basic::known_distros().join(", "));
    eprintln!("\nP.S. This fetch has superpowers. See 'rfetch --super'.")
}

fn random() {
    let chosen: isize = rand::random_range(1..7) as isize;
    match chosen {
        1 => {
            let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "rfetch".to_string());
            println!("\"i use {} btw\" - (c) skerrix", desktop)
        }
        2 => {
            if Command::new("neofetch").arg("--version").output().is_ok() {
                println!("what's neofetch?");
            }
            else if Command::new("fastfetch").arg("--version").output().is_ok() {
                println!("what's fastfetch?")
            }
            else if Command::new("hyfetch").arg("--version").output().is_ok() {
                println!("what's hyfetch?")
            }
            else {
                println!("good boy")
            }
        }
        3 => {
            println!("{}@pc ~ > paru -S opsec",whoami::username());
            println!("[paru] error: package opsec isn't found. did you mean rfetch?")
        }
        4 => {
            println!("welcome to rfetch super mode!");
            print!("please wait, installing 47 miners...");
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(3));
            println!("done");
            println!("thank you for using rfetch!")
        }
        5 => {
            let facts = ["i'm gay", "🦀", "one of its suggested original names is larpfetch", "i dont know how to make multicolored ascii, because i'm dumb"];
            println!("fun fact about rfetch: {}", facts[rand::random_range(0..facts.len() as usize)]);

        }
        6 => {
            println!("{} is not in the rfetchers file. use virfetch to add yourself", whoami::username())
        }
        _ => {eprintln!("oops")}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut distro_override: Option<String> = None;
    let mut clear_cache = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--distro" | "-d" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("Error: --distro / -d requires a value.");
                    print_usage();
                    std::process::exit(1);
                }
                distro_override = Some(args[i].clone());
            }
            "--clear-cache" => {
                clear_cache = true;
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            "--super" => {
                random();
                std::process::exit(0);
            }
            "--version" => {
                println!("rfetch v0.4.0\nby skerrix\nthanks to:\n   1. flingo\n   2. you, for using rfetch!");
                std::process::exit(0)
            }
            _ => {
                eprintln!("Error: unknown flag '{}'", args[i]);
                print_usage();
                std::process::exit(1);
            }
        }
        i += 1;
    }

    if clear_cache {
        pkgs::clear_cache();
        eprintln!("Cache cleared.");
        if distro_override.is_none() {
            std::process::exit(0);
        }
    }


    let distro_key: String = match &distro_override {
        Some(val) => val.clone(),
        None => basic::raw_os_id_or_name(),
    };


    let art_lines: Vec<String> = basic::get_ascii_art(&distro_key)
        .lines()
        .map(|l| l.replace('\t', "    "))
        .collect();
    let info_lines: Vec<String> = {


        let os_display: String = if distro_override.is_some() {
            basic::display_name_for(&distro_key).to_string()
        } else {
            basic::os()
        };

        let mut v = Vec::new();
        v.push(format!("  {}",basic::hostusr()));
        v.push(format!("{}{}", "  ".blue(), "┏━ software"));
        v.push(format!("{}{}{}", "  ".blue(), "┃ ", getform()));
        v.push(format!("{}{}{}", "  ".blue(), "┃   os: ", os_display));

        v.push(format!(
            "{}{}{}",
            "  ".cyan(),
            "┃   kernel: ",
            basic::kernel()
        ));
        v.push(format!(
        "{}{}",
        "  ┛   uptime: ",
        basic::uptime()
        ));
        v.push(format!("{}{}",  "  ".cyan(), "┏━ hardware"));
        v.push(format!("{}{}{}", "  ".cyan(), "┃   cpu: ", basic::cpu()));
        v.push(format!("{}{}{}", "  ".cyan(), "┃ 󰢮  gpu: ", basic::gpu()));
        v.push(format!(
            "{}{}{}{}{}{}{}",
            "  ┃   ram: ",
            basic::ramuse(),
            " gib / ",
            basic::ramtotal(),
            " gib (",
            basic::rampercent(),
            "%)"
        ));
        let disk_infos = basic::disks_info();
        let has_battery = basic::get_battery_charge() != 500;
        for (i, disk) in disk_infos.iter().enumerate() {
            let is_last = i == disk_infos.len() - 1;
            let prefix = if is_last && !has_battery {
                "  ┛"
            } else {
                "  ┃"
            };
            v.push(format!(
                "{}{} {} gib / {} gib ({}%)",
                prefix,
                format!("   disk ({}, {}):", disk.name, disk.filesystem),
                disk.used_gb,
                disk.total_gb,
                disk.usage_pct,
            ));
        }

        if basic::get_battery_charge() != 500 {
            if basic::get_battery_charge() <= 20 {
                v.push(format!("{}{}{}", "  ┛   battery: ", basic::get_battery_charge(), "% . charge, maybe?"));
            } else {
                v.push(format!("  ┛   battery: {}%", basic::get_battery_charge()));
            }
        }
        v
    };

    let art_width = art_lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let padding = 2usize;
    let left_width = art_width + padding;


    let (r, g, b) = basic::get_logo_color(&distro_key);

	    let max_lines = art_lines.len().max(info_lines.len());
	    for i in 0..max_lines {
	        let left = art_lines.get(i).map(|s| s.as_str()).unwrap_or("");
	        let right = info_lines.get(i).map(|s| s.as_str()).unwrap_or("");

	        if right.is_empty() {
	            if left.is_empty() {
	                println!();
	            } else {
	                println!("{}", left.truecolor(r, g, b));
	            }
	        } else {
	            if left.is_empty() {
	                println!("{:left_width$} {}", "", right);
	            } else {
	                let colored_left = left.truecolor(r, g, b);
	                let visible_w = left.chars().count();
	                let pad = left_width.saturating_sub(visible_w);
	                println!("{colored_left}{:pad$} {right}", "");
	            }
	        }
	    }
}
