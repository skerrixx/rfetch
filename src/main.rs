mod pkgs;
mod basic;
use pkgs::getform;
use colored::Colorize;
use std::env;

fn print_usage() {
    eprintln!("Usage: rfetch [--distro <distro> | -d <distro>]");
    eprintln!();
    eprintln!("Flags:");
    eprintln!("  -d, --distro <distro>   Override the detected distro art/logo");
    eprintln!();
    eprintln!("Available distros: {}", basic::known_distros().join(", "));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut distro_override: Option<String> = None;
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


    let distro_key: String = match &distro_override {
        Some(val) => val.clone(),
        None => basic::raw_os_id_or_name(),
    };


    let art_lines: Vec<&str> = basic::get_ascii_art(&distro_key).lines().collect();
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
        v.push(format!(
            "{}{}{}{}{}{}{}",
            "  ┃   disk: ",
            basic::diskuse(),
            " gib / ",
            basic::disktot(),
            " gib (",
            ((basic::diskuse() as f64 / basic::disktot() as f64) * 1000.0).round() / 10.0,
            "%)"
        ));

        if basic::get_battery_charge() != 500 {
            if basic::get_battery_charge() <= 20 {
                v.push(format!("{}{}{}", "  ┛   battery: ", basic::get_battery_charge(), "% . charge, maybe?"));
            } else {
                v.push(format!("  ┛   battery: {}%", basic::get_battery_charge()));
            }
        } else {
            v.push(format!("{}", "  ┛"));
        }
        v
    };

    let art_width = art_lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let padding = 2usize;
    let left_width = art_width + padding;


    let (r, g, b) = basic::get_logo_color(&distro_key);


    let max_lines = art_lines.len().max(info_lines.len());
    for i in 0..max_lines {
        let left = art_lines.get(i).copied().unwrap_or("");
        let right = info_lines.get(i).map(|s| s.as_str()).unwrap_or("");

        if right.is_empty() {
            if left.is_empty() {
                println!();
            } else {
                println!("{}", left.truecolor(r, g, b));
            }
        } else {
        if left.is_empty() {
            println!("{left:<left_width$} {right}");
        } else {
            let colored_left = left.truecolor(r, g, b);
            let visible_w = left.len();
            let pad = left_width.saturating_sub(visible_w);
            println!("{colored_left}{:pad$} {right}", "");
        }
        }
    }
}
