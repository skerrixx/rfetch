mod pkgs;
mod basic;
mod config;
use pkgs::getform;
use colored::Colorize;
use std::env;
use whoami;
use rand;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use std::process::Command;
use sysinfo::System;

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

fn colorize_infotext(text: &str, color: &str) -> String {
	match color.to_lowercase().as_str() {
		"red" => text.red().to_string(),
		"green" => text.green().to_string(),
		"blue" => text.blue().to_string(),
		"yellow" => text.yellow().to_string(),
		"cyan" => text.cyan().to_string(),
		"magenta" => text.magenta().to_string(),
		"purple" => text.purple().to_string(),
		"black" => text.black().to_string(),
		_ => text.to_string(),
	}
}

fn random() {
	let chosen: isize = rand::random_range(1..7) as isize;
	match chosen {
		1 => {
			let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "rfetch".to_string());
			println!("\"i use {} btw\" - (c) {}", desktop, whoami::username())
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
			let facts = ["i'm gay", "🦀", "one of its suggested original names is larpfetch", "i dont know how to make multicolored ascii, because i'm dumb (francy here, maybe i will make it cuz skerrix cant)"];            println!("fun fact about rfetch: {}", facts[rand::random_range(0..facts.len() as usize)]);

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
					eprintln!("err: --distro / -d requires a value.");
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
				println!("rfetch v0.7.5\nby skerrix and francy\nwritten using 🦀 v1.100\nthanks to:\n   1. flingo\n   2. those who installed it from the AUR\n   3. those who compiled it from source\n   4. you, for using rfetch!\n   {}", "francy is gay (im not, im bi but anyways)\n this code is licensed with GPLv3. microslop can suck our balls".black().italic());
				std::process::exit(0) // idunno changed it cuz i am named in the whole code francy soo
			}
			_ => {
				// just print the fetch if a flag is unknown (yeah skerrix youre very good at commenting(yes francy i am indeed awesome at commenting)) 
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

	let cfg = config::load_config();

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

		let hidden = |key: &str| {
			cfg.hide_info
				.iter()
				.any(|h| h.trim().to_lowercase() == key.to_lowercase())
		};

		let mut v = Vec::new();
		v.push(format!("  {}", basic::hostusr()));

		if !hidden("headers") {
			if cfg.style == "boxed" {	
				v.push(format!("{}{}", "  ".blue(), "╭──────────╮"))
			}
			else {
				v.push(format!("{}{}", "  ".blue(), "┏╸ software "))
			}
		}

		let mut software: Vec<String> = Vec::new();
		if !hidden("packages") {
			if cfg.style == "boxed" {
				software.push(format!("│  pkgs   │ {}",getform()));
			}
			else {
				software.push(format!("┃  packages: {}",getform()));
			}
		}
		if !hidden("os") {
			if cfg.style == "boxed" {
				software.push(format!("│    os   │ {}",os_display));
			}
			else {
				software.push(format!("┃  os: {}",basic::os()));
			}
		}

		let mut kernel_val = String::new();
		let mut uptime_val = String::new();
		let mut gpu_val = String::new();
		let mut disk_infos = Vec::new();
		let mut battery_charge: usize = 500;

		thread::scope(|s| {
			let h_kernel = if !hidden("kernel") {
				Some(s.spawn(|| basic::kernel()))
			} else {
				None
			};
			let h_uptime = if !hidden("uptime") {
				Some(s.spawn(|| basic::uptime()))
			} else {
				None
			};
			let h_gpu = if !hidden("gpu") {
				Some(s.spawn(|| basic::gpu()))
			} else {
				None
			};
			let h_disks = if !hidden("disk") {
				Some(s.spawn(|| basic::disks_info()))
			} else {
				None
			};
			let h_battery = s.spawn(|| basic::get_battery_charge());

			if let Some(h) = h_kernel {
				kernel_val = h.join().unwrap_or_default();
			}
			if let Some(h) = h_uptime {
				uptime_val = h.join().unwrap_or_default();
			}
			if let Some(h) = h_gpu {
				gpu_val = h.join().unwrap_or_default();
			}
			if let Some(h) = h_disks {
				disk_infos = h.join().unwrap_or_default();
			}
			battery_charge = h_battery.join().unwrap_or(500);
		});

		if !hidden("kernel") {
			if cfg.style == "boxed" {
				software.push(format!("│  kernel │ {}",kernel_val));
			}
			else {
				software.push(format!("┃  kernel: {}",kernel_val));
			}
		}
		if cfg.show_beta {
			if cfg.style == "boxed" {
				software.push(format!("│ 󰍹 de/wm  │ {}", basic::wmde()));
				software.push(format!("│  shell  │ {}", basic::shell()));
				software.push(format!("│  term   │ {}", basic::terminal()));
			}
			else {
				software.push(format!("┃ 󰍹 de/wm: {}", basic::wmde()));
				software.push(format!("┃  shell: {}", basic::shell()));
			   	software.push(format!("┃  terminal: {}", basic::terminal()));
			}
		}
		if !hidden("uptime") {
			if cfg.style == "boxed" {
				software.push(format!("│  uptime │ {}", uptime_val));
			}
			else {
				software.push(format!("┃  uptime: {}", uptime_val));
			}
		}

		for (_i, line) in software.iter().enumerate() {
			v.push(format!(
				"{}{}",
				"  ",
				colorize_infotext(line, &cfg.color_infotext)
			));
		}

		let has_battery = battery_charge != 500;

		if !hidden("headers") {
		 	if cfg.style == "boxed" {	
		 		v.push(format!("{}{}", "  ".blue(), "├──────────┤"))
		 	}
		 	else {
		 		v.push(format!("{}{}", "  ".blue(), "┏╸ hardware "))
		 	}
		}

		let sys = System::new_all();
		let mut hardware: Vec<String> = Vec::new();
		if !hidden("cpu") {
			if cfg.style == "boxed" {
				hardware.push(format!("│  cpu    │ {}", basic::cpu(&sys)));
			}
			else {
				hardware.push(format!("┃  cpu: {}", basic::cpu(&sys)));
			}
		}
		if !hidden("gpu") {
			if cfg.style == "boxed" {
				hardware.push(format!("│ 󰢮 gpu    │ {}", gpu_val));
			}
			else {
				hardware.push(format!("┃ 󰢮 gpu: {}", gpu_val));
			}
		}
		if !hidden("ram") {
			let (used, total, pct) = basic::ram_info(&sys);
			if cfg.style == "boxed" {
				hardware.push(format!(
								"│  ram    │ {} gib / {} gib ({}%)",
								 used, total, pct
							 ));
			}
			else {
				hardware.push(format!(
								"┃  ram: {} gib / {} gib ({}%)",
								used, total, pct
							));
			}
		}
		if !hidden("disk") {
			for disk in disk_infos.iter() {
				if cfg.style == "boxed" {
					hardware.push(format!(
						"│  disk   │ ({}, {}): {} gib / {} gib ({}%)",
						disk.name, disk.filesystem, disk.used_gb, disk.total_gb, disk.usage_pct,
					));
				} else {
					hardware.push(format!(
						"┃  disk ({}, {}): {} gib / {} gib ({}%)",
						disk.name, disk.filesystem, disk.used_gb, disk.total_gb, disk.usage_pct,
					));
				}
			}
		}
		if has_battery && !hidden("battery") {
			if battery_charge <= 20 {
				hardware.push(format!(" battery: {}% . charge, maybe?", battery_charge));
			} else {
				hardware.push(format!(" battery: {}%", battery_charge));
			}
		}

		for (_i, line) in hardware.iter().enumerate() {
			v.push(format!(
				"{}{}",
				"  ".cyan(),
				colorize_infotext(line, &cfg.color_infotext)
			));
		}

		if cfg.style == "boxed" {
			v.push(colorize_infotext("  ╰──────────╯",&cfg.color_infotext))
		}
		else {
			v.push(colorize_infotext("  ┛",&cfg.color_infotext))
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
					if cfg.color_ascii {
						println!("{}", left.truecolor(r, g, b));
					} else {
						println!("{}", left);
					}
				}
			} else {
				if left.is_empty() {
					println!("{:left_width$} {}", "", right);
				} else {
					let visible_w = left.chars().count();
					let pad = left_width.saturating_sub(visible_w);
					if cfg.color_ascii {
						let colored_left = left.truecolor(r, g, b);
						println!("{colored_left}{:pad$} {right}", "");
					} else {
						println!("{left}{:pad$} {right}", "");
					}
				}
			}
		}
}
