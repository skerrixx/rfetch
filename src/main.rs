mod pkgs;
mod basic;
use pkgs::getform;
use colored::Colorize;
use std::env;

fn print_usage() {
    eprintln!("Usage: rfetch [--distro <distro> | -d <distro>] [--clear-cache]");
    eprintln!();
    eprintln!("Flags:");
    eprintln!("  -d, --distro <distro>   Override the detected distro art/logo");
    eprintln!("  --clear-cache           Force rebuild of package/drives cache");
    eprintln!();
    eprintln!("Available distros: {}", basic::known_distros().join(", "));
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
