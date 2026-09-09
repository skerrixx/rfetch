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
use sysinfo::{CpuRefreshKind, System};

fn print_usage() {
	eprintln!("Usage: rfetch [options]");
	eprintln!();
	eprintln!("Options:");
	eprintln!("  -d, --distro <distro>   Override the detected distro art/logo");
	eprintln!("  --ascii <path>          Use a custom ascii art file instead of builtin");
	eprintln!("      --ascii-path, --art <path>  (aliases)");
	eprintln!("  -a, --anonymize         Hide username/hostname (for screenshots)");
	eprintln!("  --json                  Machine-readable JSON output (respects hide_info)");
	eprintln!("  -m, --minimal           One-line compact output (fast, skips gpu/disk/pkgs)");
	eprintln!("  --no-art                Show info only, no ascii art");
	eprintln!("  --logo-only             Show ascii art only, no info");
	eprintln!("  --clear-cache           Force rebuild of package/drives cache");
	eprintln!("  -h, --help              Show this help");
	eprintln!();
	eprintln!("Available distros: {}", basic::known_distros().join(", "));
	eprintln!("\nP.S. This fetch has superpowers. See 'rfetch --super'.")
}

fn expand_path(p: &str) -> String {
	let t = p.trim();
	if t == "~" || t.starts_with("~/") {
		if let Ok(home) = env::var("HOME") {
			return format!("{}{}", home, &t[1..]);
		}
	}
	if let Some(rest) = t.strip_prefix("$HOME") {
		if let Ok(home) = env::var("HOME") {
			return format!("{}{}", home, rest);
		}
	}
	t.to_string()
}

fn load_art_lines(distro_key: &str, ascii_path: Option<&str>) -> Vec<String> {
	if let Some(p) = ascii_path {
		let expanded = expand_path(p);
		match std::fs::read_to_string(&expanded) {
			Ok(content) => {
				return content.lines().map(|l| l.replace('\t', "    ")).collect();
			}
			Err(e) => {
				eprintln!("rfetch: warning: could not read ascii file {} ({}) - falling back to builtin.", expanded, e);
			}
		}
	}
	basic::get_ascii_art(distro_key)
		.lines()
		.map(|l| l.replace('\t', "    "))
		.collect()
}

fn display_hostusr(anonymize: bool) -> String {
	if anonymize {
		"anonymous ( hidden )".to_string()
	} else {
		basic::hostusr()
	}
}

/// Targeted sysinfo refresh: we only need RAM/SWAP usage + the CPU brand.
/// `System::new_all()` also scans every process (~50ms for ~300 procs);
/// this is ~20x cheaper and output-identical for what rfetch displays.
fn fresh_system() -> System {
	let mut sys = System::new();
	sys.refresh_memory();
	sys.refresh_cpu_list(CpuRefreshKind::nothing());
	sys
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
			let facts = ["i'm gay", "🦀", "one of its suggested original names is larpfetch", "i dont know how to make multicolored ascii, because i'm dumb"];            println!("fun fact about rfetch: {}", facts[rand::random_range(0..facts.len() as usize)]);

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
	let mut cli_anonymize = false;
	let mut json_output = false;
	let mut minimal_output = false;
	let mut no_art = false;
	let mut logo_only = false;
	let mut cli_ascii_path: Option<String> = None;
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
			"--ascii" | "--ascii-path" | "--art" => {
				i += 1;
				if i >= args.len() {
					eprintln!("err: --ascii requires a file path.");
					print_usage();
					std::process::exit(1);
				}
				cli_ascii_path = Some(args[i].clone());
			}
			"--anonymize" | "-a" => {
				cli_anonymize = true;
			}
			"--json" => {
				json_output = true;
			}
			"--minimal" | "-m" => {
				minimal_output = true;
			}
			"--no-art" => {
				no_art = true;
			}
			"--logo-only" => {
				logo_only = true;
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
				println!("rfetch v1.0.0\nmade with   by  skerrix and  fxrncyy\nwritten using 🦀 v1.100\nthanks to:\n   1. flingo\n   2. tromtom\n   3. those who installed it from the AUR\n   4. those who compiled it from source\n   5. you, for using rfetch!\n   {}\n{}{}{}", "francy is tuff\n".italic().black(),"this code is licensed with ","GPL-3.0".yellow(),". microslop can suck our balls");
				std::process::exit(0)
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
		let wants_display = distro_override.is_some() || json_output || minimal_output || no_art || logo_only || cli_ascii_path.is_some();
		if !wants_display {
			std::process::exit(0);
		}
	}

	if no_art && logo_only {
		eprintln!("err: --no-art and --logo-only are mutually exclusive.");
		print_usage();
		std::process::exit(1);
	}

	let cfg = config::load_config();

	let distro_key: String = match &distro_override {
		Some(val) => val.clone(),
		None => basic::raw_os_id_or_name(),
	};

	let anonymize = cli_anonymize || cfg.anonymize;
	let ascii_path_opt: Option<String> = cli_ascii_path.or_else(|| cfg.ascii_path.clone());
	let ascii_path_ref: Option<&str> = ascii_path_opt.as_deref();

	let hidden = |key: &str| {
		cfg.hide_info
			.iter()
			.any(|h| h.trim().to_lowercase() == key.to_lowercase())
	};
	// os_age is shown unless *either* "os_age" or "age" is hidden.
	let os_age_hidden = hidden("os_age") || hidden("age");
	// promoted-to-stable (ex-beta): de/wm, shell, terminal are normal hideable rows now.
	let dewm_hidden = hidden("de/wm") || hidden("de_wm") || hidden("de") || hidden("wm");
	let shell_hidden = hidden("shell");
	let term_hidden = hidden("terminal") || hidden("term");
	let boot_hidden = hidden("boot");
	let swap_hidden = hidden("swap");
	let load_hidden = hidden("load") || hidden("loadavg") || hidden("load_avg");
	let procs_hidden = hidden("processes") || hidden("procs") || hidden("proc");

	let os_display: String = if distro_override.is_some() {
		basic::display_name_for(&distro_key).to_string()
	} else {
		basic::os()
	};
	let host_display: String = display_hostusr(anonymize);

	// --logo-only fast path: no info fetching at all
	if logo_only {
		let art_lines = load_art_lines(&distro_key, ascii_path_ref);
		let (r, g, b) = basic::get_logo_color(&distro_key);
		for line in &art_lines {
			if cfg.color_ascii {
				println!("{}", line.truecolor(r, g, b));
			} else {
				println!("{}", line);
			}
		}
		std::process::exit(0);
	}

	// --minimal fast path: skip heavy collectors (pkgs, gpu, disks, battery)
	if minimal_output {
		let kernel_val = if !hidden("kernel") { basic::kernel() } else { String::new() };
		let uptime_val = if !hidden("uptime") { basic::uptime() } else { String::new() };
		let os_age_val = if !os_age_hidden { basic::os_age() } else { String::new() };
		let boot_val = if !boot_hidden { basic::boot_time() } else { String::new() };
		let load_val = if !load_hidden { basic::load_avg() } else { String::new() };
		let procs_val = if !procs_hidden { basic::process_count() } else { None };
		let need_sys = !hidden("cpu") || !hidden("ram") || !swap_hidden;
		let sys = if need_sys { Some(fresh_system()) } else { None };
		let cpu_val = if !hidden("cpu") {
			sys.as_ref().map(|s| basic::cpu(s)).unwrap_or_default()
		} else {
			String::new()
		};
		let ram_str = if !hidden("ram") {
			if let Some(s) = sys.as_ref() {
				let (used, total, pct) = basic::ram_info(s);
				format!("{} gib / {} gib ({}%)", used, total, pct)
			} else {
				String::new()
			}
		} else {
			String::new()
		};
		let swap_str = if !swap_hidden {
			if let Some(s) = sys.as_ref() {
				let (used, total, pct) = basic::swap_info(s);
				format!("{} gib / {} gib ({}%)", used, total, pct)
			} else {
				String::new()
			}
		} else {
			String::new()
		};
		let user_host = if anonymize {
			"anonymous@hidden".to_string()
		} else {
			format!("{}@{}", whoami::username(), whoami::fallible::hostname().unwrap_or_default())
		};
		let mut parts: Vec<String> = vec![user_host];
		if !hidden("os") {
			parts.push(format!("os: {}", os_display));
		}
		if !hidden("kernel") && !kernel_val.is_empty() {
			parts.push(format!("kernel: {}", kernel_val));
		}
		if !dewm_hidden {
			parts.push(format!("de/wm: {}", basic::wmde()));
		}
		if !shell_hidden {
			parts.push(format!("shell: {}", basic::shell()));
		}
		{
			let t = basic::terminal();
			if !term_hidden && !t.is_empty() {
				parts.push(format!("term: {}", t));
			}
		}
		if !hidden("uptime") && !uptime_val.is_empty() {
			parts.push(format!("uptime: {}", uptime_val));
		}
		if !boot_hidden && !boot_val.is_empty() {
			parts.push(format!("boot: {}", boot_val));
		}
		if !os_age_hidden && !os_age_val.is_empty() {
			parts.push(format!("age: {}", os_age_val));
		}
		if !hidden("cpu") {
			parts.push(format!("cpu: {}", cpu_val));
		}
		if !hidden("ram") {
			parts.push(format!("ram: {}", ram_str));
		}
		if !swap_hidden {
			parts.push(format!("swap: {}", swap_str));
		}
		if !load_hidden && !load_val.is_empty() {
			parts.push(format!("load: {}", load_val));
		}
		if let Some(n) = procs_val {
			parts.push(format!("processes: {}", n));
		}
		println!("{}", parts.join(" | "));
		std::process::exit(0);
	}

	// full fetch (for --json / --no-art / full). respects hide_info to skip work.
	let mut kernel_val = String::new();
	let mut uptime_val = String::new();
	let mut os_age_val = String::new();
	let mut boot_val = String::new();
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
		let h_os_age = if !os_age_hidden {
			Some(s.spawn(|| basic::os_age()))
		} else {
			None
		};
		let h_boot = if !boot_hidden {
			Some(s.spawn(|| basic::boot_time()))
		} else {
			None
		};
		let h_gpu = if !hidden("gpu") {
			Some(s.spawn(|| {
				// gpu probing does DRM init (~18ms); reuse the 1h pkg-cache when fresh.
				pkgs::cached_gpu().unwrap_or_else(basic::gpu)
			}))
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
		if let Some(h) = h_os_age {
			os_age_val = h.join().unwrap_or_default();
		}
		if let Some(h) = h_boot {
			boot_val = h.join().unwrap_or_default();
		}
		if let Some(h) = h_gpu {
			gpu_val = h.join().unwrap_or_default();
		}
		if let Some(h) = h_disks {
			disk_infos = h.join().unwrap_or_default();
		}
		battery_charge = h_battery.join().unwrap_or(500);
	});

	let packages_val: Option<String> = if !hidden("packages") { Some(getform()) } else { None };
	let need_sys = !hidden("cpu") || !hidden("ram") || !swap_hidden;
	let sys = if need_sys { fresh_system() } else { System::new() };
	let cpu_val: String = if !hidden("cpu") { basic::cpu(&sys) } else { String::new() };
	let ram_vals: Option<(String, String, String)> = if !hidden("ram") { Some(basic::ram_info(&sys)) } else { None };
	let swap_vals: Option<(String, String, String)> = if !swap_hidden { Some(basic::swap_info(&sys)) } else { None };
	let load_val: String = if !load_hidden { basic::load_avg() } else { String::new() };
	let procs_val: Option<usize> = if !procs_hidden { basic::process_count() } else { None };
	let wmde_val: String = if !dewm_hidden { basic::wmde() } else { String::new() };
	let shell_val: String = if !shell_hidden { basic::shell() } else { String::new() };
	let term_val: String = if !term_hidden { basic::terminal() } else { String::new() };

	if json_output {
		let user_name = if anonymize { "anonymous".to_string() } else { whoami::username() };
		let hostname = if anonymize { "hidden".to_string() } else { whoami::fallible::hostname().unwrap_or_default() };
		let mut obj = serde_json::Map::new();
		obj.insert("rfetch".to_string(), serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string()));
		obj.insert("user".to_string(), serde_json::Value::String(user_name));
		obj.insert("hostname".to_string(), serde_json::Value::String(hostname));
		obj.insert("distro".to_string(), serde_json::Value::String(distro_key.clone()));
		obj.insert("os".to_string(), serde_json::Value::String(os_display.clone()));
		if let Some(p) = packages_val.clone() {
			obj.insert("packages".to_string(), serde_json::Value::String(p));
		}
		if !hidden("kernel") {
			obj.insert("kernel".to_string(), serde_json::Value::String(kernel_val.clone()));
		}
		if !dewm_hidden {
			obj.insert("de_wm".to_string(), serde_json::Value::String(wmde_val.clone()));
		}
		if !shell_hidden {
			obj.insert("shell".to_string(), serde_json::Value::String(shell_val.clone()));
		}
		if !term_hidden && !term_val.is_empty() {
			obj.insert("terminal".to_string(), serde_json::Value::String(term_val.clone()));
		}
		if !hidden("uptime") {
			obj.insert("uptime".to_string(), serde_json::Value::String(uptime_val.clone()));
		}
		if !boot_hidden {
			obj.insert("boot".to_string(), serde_json::Value::String(boot_val.clone()));
		}
		if !os_age_hidden {
			obj.insert("os_age".to_string(), serde_json::Value::String(os_age_val.clone()));
		}
		if !hidden("cpu") {
			obj.insert("cpu".to_string(), serde_json::Value::String(cpu_val.clone()));
		}
		if !hidden("gpu") {
			obj.insert("gpu".to_string(), serde_json::Value::String(gpu_val.clone()));
		}
		if let Some((used, total, pct)) = ram_vals.clone() {
			obj.insert("ram".to_string(), serde_json::json!({"used_gib": used, "total_gib": total, "pct": pct}));
		}
		if let Some((used, total, pct)) = swap_vals.clone() {
			obj.insert("swap".to_string(), serde_json::json!({"used_gib": used, "total_gib": total, "pct": pct}));
		}
		if !load_hidden {
			obj.insert("load".to_string(), serde_json::Value::String(load_val.clone()));
		}
		if !procs_hidden {
			match procs_val {
				Some(n) => { obj.insert("processes".to_string(), serde_json::json!(n)); }
				None => { obj.insert("processes".to_string(), serde_json::Value::Null); }
			}
		}
		if !hidden("disk") {
			let arr: Vec<serde_json::Value> = disk_infos.iter().map(|d| {
				serde_json::json!({"name": d.name, "filesystem": d.filesystem, "used_gb": d.used_gb, "total_gb": d.total_gb, "usage_pct": d.usage_pct})
			}).collect();
			obj.insert("disks".to_string(), serde_json::Value::Array(arr));
		}
		if !hidden("battery") {
			if battery_charge != 500 {
				obj.insert("battery".to_string(), serde_json::json!(battery_charge));
			} else {
				obj.insert("battery".to_string(), serde_json::Value::Null);
			}
		}
		println!("{}", serde_json::to_string_pretty(&serde_json::Value::Object(obj)).unwrap_or_default());
		std::process::exit(0);
	}

	let art_lines: Vec<String> = load_art_lines(&distro_key, ascii_path_ref);
	let info_lines: Vec<String> = {

		let mut v = Vec::new();
		v.push(format!("  {}", host_display));

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
			let pkgs = packages_val.clone().unwrap_or_default();
			if cfg.style == "boxed" {
				software.push(format!("│  pkgs   │ {}", pkgs));
			}
			else {
				software.push(format!("┃  packages: {}", pkgs));
			}
		}
		if !hidden("os") {
			if cfg.style == "boxed" {
				software.push(format!("│    os   │ {}",os_display));
			}
			else {
				software.push(format!("┃  os: {}", os_display));
			}
		}

		if !hidden("kernel") {
			if cfg.style == "boxed" {
				software.push(format!("│  kernel │ {}",kernel_val));
			}
			else {
				software.push(format!("┃  kernel: {}",kernel_val));
			}
		}
		if !dewm_hidden {
			if cfg.style == "boxed" {
				software.push(format!("│ 󰍹 de/wm  │ {}", wmde_val));
			}
			else {
				software.push(format!("┃ 󰍹 de/wm: {}", wmde_val));
			}
		}
		if !shell_hidden {
			if cfg.style == "boxed" {
				software.push(format!("│  shell  │ {}", shell_val));
			}
			else {
				software.push(format!("┃  shell: {}", shell_val));
			}
		}
		if !term_hidden && !term_val.is_empty() {
			if cfg.style == "boxed" {
				software.push(format!("│  term   │ {}", term_val));
			}
			else {
			   	software.push(format!("┃  terminal: {}", term_val));
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
		if !boot_hidden {
			if cfg.style == "boxed" {
				software.push(format!("│  boot   │ {}", boot_val));
			}
			else {
				software.push(format!("┃  boot: {}", boot_val));
			}
		}
		if !os_age_hidden {
			if cfg.style == "boxed" {
				software.push(format!("│ 󰃭 age    │ {}", os_age_val));
			}
			else {
				software.push(format!("┃ 󰃭 age: {}", os_age_val));
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

		let mut hardware: Vec<String> = Vec::new();
		if !hidden("cpu") {
			if cfg.style == "boxed" {
				hardware.push(format!("│  cpu    │ {}", cpu_val));
			}
			else {
				hardware.push(format!("┃  cpu: {}", cpu_val));
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
		if let Some((used, total, pct)) = ram_vals.clone() {
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
		if let Some((used, total, pct)) = swap_vals.clone() {
			if cfg.style == "boxed" {
				hardware.push(format!(
								"│ 󰍛 swap   │ {} gib / {} gib ({}%)",
								 used, total, pct
							 ));
			}
			else {
				hardware.push(format!(
								"┃ 󰍛 swap: {} gib / {} gib ({}%)",
								used, total, pct
							));
			}
		}
		if !load_hidden {
			if cfg.style == "boxed" {
				hardware.push(format!("│  load   │ {}", load_val));
			}
			else {
				hardware.push(format!("┃  load: {}", load_val));
			}
		}
		if !procs_hidden {
			let procs_str = procs_val.map(|n| n.to_string()).unwrap_or_else(|| "unknown".to_string());
			if cfg.style == "boxed" {
				hardware.push(format!("│  procs  │ {}", procs_str));
			}
			else {
				hardware.push(format!("┃  processes: {}", procs_str));
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

	if no_art {
		for line in &info_lines {
			println!("{}", line);
		}
		std::process::exit(0);
	}

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
