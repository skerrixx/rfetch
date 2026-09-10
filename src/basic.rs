use std::process::Command;
use std::path::Path;
use std::env;
use std::collections::HashSet;
use sysinfo::System;
use gfxinfo;
use whoami;
use starship_battery::Manager;
use starship_battery::units::ratio::percent;

#[path = "logos/mod.rs"]
mod logos;

pub use logos::{display_name_for, get_ascii_art, get_logo_color, known_distros};

pub fn is_termux() -> bool {
    if env::var("TERMUX_VERSION").is_ok() {
        return true;
    }
    if let Ok(prefix) = env::var("PREFIX") {
        if prefix.contains("com.termux") {
            return true;
        }
    }
    std::fs::exists("/data/data/com.termux").unwrap_or(false)
}

fn os_id_or_name() -> String {
    if is_termux() {
        return String::from("android");
    }
    if std::fs::exists("/bedrock/strata/bedrock/etc/os-release").unwrap() {
        let content = match std::fs::read_to_string("/bedrock/strata/bedrock/etc/os-release") {
            Ok(c) => c,
            Err(_) => return String::from(""),
        };

        fn extract_value(content: &str, key: &str) -> Option<String> {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix(&format!("{}=", key)) {
                    let v = rest.trim();
                    return Some(
                        v.trim_start_matches('"')
                            .trim_end_matches('"')
                            .to_string(),
                    );
                }
            }
            None
        }

        extract_value(&content, "ID")
            .or_else(|| extract_value(&content, "NAME"))
            .unwrap_or_default()
    } else if std::fs::exists("/etc/os-release").unwrap() {
        let content = match std::fs::read_to_string("/etc/os-release") {
            Ok(c) => c,
            Err(_) => return String::from(""),
        };

        fn extract_value(content: &str, key: &str) -> Option<String> {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix(&format!("{}=", key)) {
                    let v = rest.trim();
                    return Some(
                        v.trim_start_matches('"')
                            .trim_end_matches('"')
                            .to_string(),
                    );
                }
            }
            None
        }

        extract_value(&content, "ID")
            .or_else(|| extract_value(&content, "NAME"))
            .unwrap_or_default()
    }
    else {
        let os = String::from_utf8(Command::new("uname").arg("s").output().expect("").stdout).expect("").to_string().to_lowercase();
        return os;
    }
}


pub fn cpu(sys: &System) -> String {
    if let Some(first_cpu) = sys.cpus().first() {
        first_cpu.brand().to_string()
    } else {
        String::new()
    }
}

pub fn raw_os_id_or_name() -> String {
    os_id_or_name()
}


pub fn os() -> String {
    logos::display_name_for(&os_id_or_name()).to_string()
}

pub fn ram_info(sys: &System) -> (String, String, String) {
    let used_bytes = sys.used_memory();
    let total_bytes = sys.total_memory();
    let used_gb = ((used_bytes as f32 / 1024.0 / 1024.0 / 1024.0) * 10.0).ceil() / 10.0;
    let total_gb = ((total_bytes as f32 / 1024.0 / 1024.0 / 1024.0) * 10.0).ceil() / 10.0;
    let pct = if total_gb > 0.0 {
        ((used_gb / total_gb) * 100.0).round() as u32
    } else {
        0
    };
    (used_gb.to_string(), total_gb.to_string(), pct.to_string())
}

pub struct DiskInfo {
    pub name: String,
    pub filesystem: String,
    pub used_gb: f64,
    pub total_gb: f64,
    pub usage_pct: f64,
}

pub fn disks_info() -> Vec<DiskInfo> {
    let gb = 1024.0 * 1024.0 * 1024.0;
    let mut result = Vec::new();

    let df_output = Command::new("df")
        .arg("-B1")
        .arg("--exclude-type=tmpfs")
        .arg("--exclude-type=devtmpfs")
        .arg("--exclude-type=squashfs")
        .arg("--exclude-type=overlay")
        .arg("--exclude-type=proc")
        .arg("--exclude-type=sysfs")
        .arg("--exclude-type=cgroup")
        .arg("--exclude-type=devpts")
        .arg("--exclude-type=hugetlbfs")
        .arg("--exclude-type=mqueue")
        .arg("--exclude-type=pstore")
        .arg("--exclude-type=securityfs")
        .arg("--exclude-type=efivarfs")
        .arg("--exclude-type=bpf")
        .arg("--exclude-type=tracefs")
        .arg("--exclude-type=debugfs")
        .arg("--exclude-type=configfs")
        .arg("--exclude-type=fusectl")
        .arg("--exclude-type=autofs")
        .arg("--output=source,fstype,target,size,used,avail")
        .output()
        .ok();

    if let Some(output) = df_output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut lines = stdout.lines();
            lines.next();
            for line in lines {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 6 {
                    continue;
                }

                let source = parts[0];
                let fstype = parts[1];
                let target = parts[2];


                if target.starts_with("/boot") {
                    continue;
                }
                if !source.starts_with("/dev/") {
                    continue;
                }

                if fstype.starts_with("fuse.") || fstype == "fuse" {
                    continue;
                }

                let total_bytes: f64 = match parts[3].parse() { Ok(v) => v, Err(_) => continue };
                let used_bytes: f64 = match parts[4].parse() { Ok(v) => v, Err(_) => continue };

                if total_bytes <= 0.0 {
                    continue;
                }

                let total_gb = total_bytes / gb;
                let used_gb = used_bytes / gb;
                let pct = (used_bytes / total_bytes) * 100.0;


                let name = source.strip_prefix("/dev/").unwrap_or(source).to_string();

                result.push(DiskInfo {
                    name,
                    filesystem: fstype.to_string(),
                    used_gb: (used_gb * 10.0).round() / 10.0,
                    total_gb: (total_gb * 10.0).round() / 10.0,
                    usage_pct: (pct * 10.0).round() / 10.0,
                });
            }

            let mut seen = HashSet::new();
            result.retain(|d| seen.insert(d.name.clone()));

            if !result.is_empty() {
                return result;
            }
        }
    }

    let disks = sysinfo::Disks::new_with_refreshed_list();
    let mut seen = HashSet::new();
    for disk in disks.list() {
        let fs_name = disk.file_system().to_string_lossy();
        if fs_name == "fuse" || fs_name.starts_with("fuse.") {
            continue;
        }
        if fs_name == "fusectl" {
            continue;
        }
        let skip_fs: &[&str] = &[
            "tmpfs", "devtmpfs", "squashfs", "overlay", "proc", "sysfs",
            "cgroup", "devpts", "hugetlbfs", "mqueue", "pstore",
            "securityfs", "efivarfs", "bpf", "tracefs", "debugfs",
            "configfs", "autofs", "efiivarfs",
        ];
        if skip_fs.contains(&fs_name.as_ref()) {
            continue;
        }

        let mountpoint = disk.mount_point().to_string_lossy().to_string();
        if mountpoint.starts_with("/boot") {
            continue;
        }

        let total = disk.total_space() as f64 / gb;
        let avail = disk.available_space() as f64 / gb;
        let used = total - avail;
        let pct = if total > 0.0 { (used / total) * 100.0 } else { 0.0 };

        let dev_name = disk.name().to_string_lossy().to_string();
        let name = if dev_name.starts_with('/') {
            dev_name.strip_prefix("/dev/").unwrap_or(&dev_name)
        } else {
            &dev_name
        };

        let fstype = disk.file_system().to_string_lossy().to_string();

        let dev_name = name.to_string();
        if !seen.insert(dev_name.clone()) {
            continue;
        }
        result.push(DiskInfo {
            name: dev_name,
            filesystem: fstype,
            used_gb: (used * 10.0).round() / 10.0,
            total_gb: (total * 10.0).round() / 10.0,
            usage_pct: (pct * 10.0).round() / 10.0,
        });
    }

    result
}

pub fn wmde() -> String {
	let unp_de = env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "rfetch".to_string());
	match unp_de.to_lowercase().as_str() {
		"gnome" => String::from(" gnome"),
		"kde" | "plasma" => String::from(" kde"),
		"niri" => String::from(" niri"),
		"hyprland" => String::from(" hypr"),
		"xfce" => String::from(" xfce"),
		"sway" => String::from(" sway"),
		"i3" => String::from(" i3"),
		"mango" | "mangowm" => String::from("󱁆 mango"),
		"cinnamon" | "x-cinnamon" => String::from(" cinnamon"),
		_ => format!(" {}", unp_de.to_lowercase())
	}
}
pub fn kernel() -> String {
    if is_termux() {
        if let Ok(ver) = std::fs::read_to_string("/proc/version") {
            let version = ver.split_whitespace().nth(2).unwrap_or("unknown");
            return format!("󰌽 linux {}", version);
        }
    }
    std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| format!("󰌽 linux {}", s.trim()))
        .unwrap_or_else(|_| {
            let output = Command::new("uname").arg("-sr").output().expect("");
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        })
}
pub fn shell() -> String {
    let mut sh_unp = String::from("");
    if let Ok(shell_path) = std::env::var("SHELL") {
        if let Some(name) = Path::new(&shell_path).file_name() {
            sh_unp = name.to_string_lossy().to_string();
        }
    };

    if let Ok(passwd) = std::fs::read_to_string("/etc/passwd") {
        let username = std::env::var("USER").unwrap_or_default();
        for line in passwd.lines() {
            if line.starts_with(&format!("{}:", username)) {
                if let Some(shell) = line.split(':').last() {
                    sh_unp = Path::new(shell)
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                }
            }
        }
    };
    if sh_unp.trim().is_empty() {
        sh_unp = String::from("unknown");
    };
    match sh_unp.as_str() {
        "fish" => " fish".to_string(),
        "bash" => " bash".to_string(),
        "zsh" => " zsh".to_string(),
        "sh" => " sh".to_string(),
        _ => " unknown".to_string()
    }
}

pub fn terminal() -> String {
    let mut unp_t = String::new();
    if let Some(term_program) = std::env::var("TERM_PROGRAM")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
    {
        unp_t = term_program;
    }

    if let Some(term) = std::env::var("TERM")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
    {
        unp_t = term;
    }
    if !unp_t.is_empty() {
        return match unp_t.to_lowercase().as_str() {
            "alacritty" => "󱐋 alacritty".to_string(),
            "xterm-kitty" => " kitty".to_string(),
            "tabby" => " tabby".to_string(),
            "foot" => " foot".to_string(),
            "xterm-256color" => " DE terminal".to_string(),
            "xterm-ghostty" => "󰊠 ghostty".to_string(),
            _ => format!(" {}", unp_t.to_lowercase())
        }
    }

    String::new()
}
pub fn gpu() -> String {
    if let Ok(gpu) = gfxinfo::active_gpu() {
        return gpu.model().to_string();
    }

    if is_termux() {
        if let Ok(soc) = std::fs::read_to_string("/sys/devices/soc0/soc_id") {
            let id = soc.trim().to_string();
            return format!("SoC (id: {})", id);
        }
        if let Ok(hw) = std::fs::read_to_string("/sys/devices/soc0/machine") {
            return hw.trim().to_string();
        }
    }

    "none found, maybe integrated".to_string()
}

pub fn hostusr() -> String {
    format!(
        "{} ( {} )",
        whoami::username(),
        whoami::fallible::hostname().unwrap_or_default()
    )
}

pub fn uptime() -> String {
    std::fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|content| {
            let secs: f64 = content.split_whitespace().next()?.parse().ok()?;
            let total_secs = secs as u64;
            let days = total_secs / 86400;
            let hours = (total_secs % 86400) / 3600;
            let mins = (total_secs % 3600) / 60;
            let mut parts = Vec::new();
            if days > 0 {
                parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
            }
            if hours > 0 {
                parts.push(format!("{} hour{}", hours, if hours == 1 { "" } else { "s" }));
            }
            if mins > 0 || parts.is_empty() {
                parts.push(format!("{} minute{}", mins, if mins == 1 { "" } else { "s" }));
            }
            Some(parts.join(", "))
        })
        .unwrap_or_else(|| {
            let output = Command::new("uptime").arg("-p").output().expect("");
            String::from_utf8_lossy(&output.stdout).trim().to_string().replace("up ", "")
        })
}

pub fn get_battery_charge() -> usize {
    if is_termux() {
        if let Ok(cap) = std::fs::read_to_string("/sys/class/power_supply/battery/capacity") {
            if let Ok(pct) = cap.trim().parse::<usize>() {
                return pct;
            }
        }
        return 500;
    }

    let manager = match Manager::new() {
        Ok(m) => m,
        Err(_) => return 500,
    };

    if let Ok(mut battery_list) = manager.batteries() {
        if let Some(Ok(battery)) = battery_list.next() {
            let raw_percent: f32 = battery.state_of_charge().get::<percent>();
            return raw_percent as usize;
        }
    }

    500
}

// --- os age (time since install) ---

use std::time::{SystemTime, UNIX_EPOCH};

fn birth_time_of(path: &str) -> Option<SystemTime> {
    std::fs::metadata(path).ok()?.created().ok()
}

fn modified_time_of(path: &str) -> Option<SystemTime> {
    std::fs::metadata(path).ok()?.modified().ok()
}

fn stat_birth_fallback(path: &str) -> Option<SystemTime> {
    // `stat -c %W` prints birth as unix secs, 0 if unknown.
    let out = Command::new("stat").arg("-c").arg("%W").arg(path).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let secs: u64 = s.parse().ok()?;
    if secs == 0 {
        return None;
    }
    UNIX_EPOCH.checked_add(std::time::Duration::from_secs(secs))
}

/// Try to parse the first timestamp in /var/log/pacman.log (arch btw).
/// Lines look like: [2026-09-07T20:47:00+0200] [PACMAN] Running ...
fn pacman_log_install_time() -> Option<SystemTime> {
    let content = std::fs::read_to_string("/var/log/pacman.log").ok()?;
    for line in content.lines() {
        let line = line.trim();
        if !line.starts_with('[') || line.len() < 11 {
            continue;
        }
        // extract YYYY-MM-DD inside brackets
        let end = line.find(']')?;
        let inner = &line[1..end];
        // inner is like 2026-09-07T20:47:00+0200, take date part
        if inner.len() < 10 {
            continue;
        }
        let date_part = &inner[0..10];
        let mut parts = date_part.split('-');
        let y: i32 = parts.next()?.parse().ok()?;
        let m: u32 = parts.next()?.parse().ok()?;
        let d: u32 = parts.next()?.parse().ok()?;
        if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
            continue;
        }
        let days = days_from_civil(y, m, d);
        let secs = days * 86400;
        if secs < 0 {
            continue;
        }
        return UNIX_EPOCH.checked_add(std::time::Duration::from_secs(secs as u64));
    }
    None
}

fn days_from_civil(y: i32, m: u32, d: u32) -> i64 {
    // Howard Hinnant's days_from_civil, days since 1970-01-01
    let y = if m <= 2 { y - 1 } else { y } as i64;
    let m = m as i64;
    let d = d as i64;
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn civil_from_days(z: i64) -> (i32, u32, u32) {
    // Howard Hinnant's civil_from_days, inverse of above
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let mut y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    if m <= 2 {
        y += 1;
    }
    (y as i32, m as u32, d as u32)
}

fn format_install_date(t: SystemTime) -> Option<String> {
    let secs = t.duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;
    let days = secs.div_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    Some(format!("{:04}-{:02}-{:02}", y, m, d))
}

fn format_age_duration(secs: u64) -> String {
    let days = secs / 86400;
    if days == 0 {
        let hours = secs / 3600;
        if hours == 0 {
            let mins = secs / 60;
            if mins <= 1 {
                return "just now".to_string();
            }
            return format!("{} minutes", mins);
        }
        if hours == 1 {
            return "1 hour".to_string();
        }
        return format!("{} hours", hours);
    }
    if days == 1 {
        return "1 day".to_string();
    }
    if days < 30 {
        return format!("{} days", days);
    }
    if days < 365 {
        let months = days / 30;
        let rem = days % 30;
        let m_word = if months == 1 { "month" } else { "months" };
        if rem == 0 {
            return format!("{} {}", months, m_word);
        }
        let d_word = if rem == 1 { "day" } else { "days" };
        return format!("{} {} {} {}", months, m_word, rem, d_word);
    }
    let years = days / 365;
    let rem = days % 365;
    let months = rem / 30;
    let d = rem % 30;
    let y_word = if years == 1 { "year" } else { "years" };
    let mut s = format!("{} {}", years, y_word);
    if months > 0 {
        s.push_str(&format!(" {} {}", months, if months == 1 { "month" } else { "months" }));
    }
    if d > 0 {
        s.push_str(&format!(" {} {}", d, if d == 1 { "day" } else { "days" }));
    }
    s
}

pub fn os_install_time() -> Option<SystemTime> {
    let mut candidates: Vec<SystemTime> = Vec::new();

    // 1. birth time of / (filesystem creation ~= install). best generic signal.
    if let Some(t) = birth_time_of("/").or_else(|| stat_birth_fallback("/")) {
        candidates.push(t);
    }
    // 2. birth of machine-id files (generated at install)
    for p in ["/etc/machine-id", "/var/lib/dbus/machine-id"] {
        if let Some(t) = birth_time_of(p).or_else(|| stat_birth_fallback(p)) {
            candidates.push(t);
        }
    }
    // 3. birth of pacman log (arch)
    if let Some(t) = birth_time_of("/var/log/pacman.log") {
        candidates.push(t);
    }

    if !candidates.is_empty() {
        // oldest = most likely the install
        candidates.sort();
        return candidates.into_iter().next();
    }

    // 4. fallbacks when btime is unsupported (some fs/kernels return 0/err):
    // mtime of write-once-at-install files
    for p in [
        "/etc/machine-id",
        "/var/lib/dbus/machine-id",
        "/root/anaconda-ks.cfg",
        "/var/log/installer/syslog",
        "/etc/arch-release",
    ] {
        if let Some(t) = modified_time_of(p) {
            candidates.push(t);
        }
    }
    // 5. parse pacman log content (arch btw^2)
    if let Some(t) = pacman_log_install_time() {
        candidates.push(t);
    }

    if candidates.is_empty() {
        return None;
    }
    candidates.sort();
    candidates.into_iter().next()
}

pub fn os_age() -> String {
    let install = match os_install_time() {
        Some(t) => t,
        None => return "unknown".to_string(),
    };
    let now = SystemTime::now();
    let dur = match now.duration_since(install) {
        Ok(d) => d,
        Err(_) => return "unknown".to_string(),
    };
    let age = format_age_duration(dur.as_secs());
    match format_install_date(install) {
        Some(date) => format!("{} (installed {})", age, date),
        None => age,
    }
}

// --- promoted-to-stable hardware gaps: swap / load / processes / boot ---

pub fn swap_info(sys: &System) -> (String, String, String) {
    let used_bytes = sys.used_swap();
    let total_bytes = sys.total_swap();
    let used_gb = ((used_bytes as f32 / 1024.0 / 1024.0 / 1024.0) * 10.0).ceil() / 10.0;
    let total_gb = ((total_bytes as f32 / 1024.0 / 1024.0 / 1024.0) * 10.0).ceil() / 10.0;
    let pct = if total_gb > 0.0 {
        ((used_gb / total_gb) * 100.0).round() as u32
    } else {
        0
    };
    (used_gb.to_string(), total_gb.to_string(), pct.to_string())
}

pub fn load_avg() -> String {
    if let Ok(content) = std::fs::read_to_string("/proc/loadavg") {
        let mut parts = content.split_whitespace();
        if let (Some(one), Some(five), Some(fifteen)) = (parts.next(), parts.next(), parts.next()) {
            return format!("{}, {}, {}", one, five, fifteen);
        }
    }
    let avg = System::load_average();
    format!("{:.2}, {:.2}, {:.2}", avg.one, avg.five, avg.fifteen)
}

pub fn process_count() -> Option<usize> {
    // counting /proc numeric dirs = processes (threads excluded, unlike loadavg total)
    if let Ok(entries) = std::fs::read_dir("/proc") {
        let mut n = 0usize;
        for e in entries.flatten() {
            let name = e.file_name();
            let s = name.to_string_lossy();
            if !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) {
                n += 1;
            }
        }
        if n > 0 {
            return Some(n);
        }
    }
    // fallback: total from /proc/loadavg 4th field ("3/786" -> 786, threads included)
    if let Ok(content) = std::fs::read_to_string("/proc/loadavg") {
        let fourth = content.split_whitespace().nth(3).unwrap_or("");
        if let Some(total) = fourth.split('/').nth(1) {
            if let Ok(v) = total.parse::<usize>() {
                return Some(v);
            }
        }
    }
    None
}

pub fn boot_time() -> String {
    // `uptime -s` already prints local boot time ("2026-09-09 08:50:25")
    if let Ok(out) = Command::new("uptime").arg("-s").output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() && s != "unknown" {
                return s;
            }
        }
    }
    // fallback: sysinfo boot timestamp formatted via `date` (local tz)
    let ts = System::boot_time();
    if ts > 0 {
        if let Ok(out) = Command::new("date")
            .arg("-d")
            .arg(format!("@{}", ts))
            .arg("+%F %T")
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !s.is_empty() {
                    return s;
                }
            }
        }
    }
    "unknown".to_string()
}

