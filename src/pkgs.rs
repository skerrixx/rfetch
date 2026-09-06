use std::process::Command;
use std::thread;
use std::time::{SystemTime, Duration};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct PackageCache {
    debian: Option<usize>,
    arch: Option<usize>,
    redhat: Option<usize>,
    void: Option<usize>,
    gentoo: Option<usize>,
    alpine: Option<usize>,
    flatpak: Option<usize>,
    suse: Option<usize>,
    netbsd: Option<usize>,
    termux: Option<usize>,
    timestamp: SystemTime,
}

fn count_packages(cmd: &str, args: &[&str]) -> Option<usize> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|o| {
            if !o.status.success() {
                return None;
            }
            let count = String::from_utf8_lossy(&o.stdout).lines().count();
            if count > 0 { Some(count) } else { None }
        })
}

fn cache_path() -> String {
    if let Ok(tmpdir) = std::env::var("TMPDIR") {
        format!("{}/rfetch_packages.json", tmpdir)
    } else {
        "/tmp/rfetch_packages.json".to_string()
    }
}

fn get_installed_packages_parallel() -> String {
    let cp = cache_path();
    if let Ok(data) = std::fs::read_to_string(&cp) {
        if let Ok(cache) = serde_json::from_str::<PackageCache>(&data) {
            if cache.timestamp.elapsed().unwrap_or_default() < Duration::from_secs(3600) {
                return format_package_string(&cache);
            }
        }
    }

    let debian = thread::spawn(|| count_packages("dpkg", &["--get-selections"]));
    let arch = thread::spawn(|| count_packages("pacman", &["-Q"]));
    let redhat = thread::spawn(|| count_packages("dnf", &["list", "--installed"]));
    let alpine = thread::spawn(|| count_packages("apk", &["info"]));
    let void = thread::spawn(|| count_packages("xbps-query", &["-l"]));
    let flatpak = thread::spawn(|| count_packages("flatpak", &["list"]));
    let gentoo = thread::spawn(|| count_packages("qlist", &["-Iv"]));
    let suse = thread::spawn(|| count_packages("zypper", &["se", "-i"]));
    let netbsd = thread::spawn(|| count_packages("pkg_info", &["-q"]));
    let termux = thread::spawn(|| count_packages("dpkg-query", &["-f", "${Status}\n", "--show"]));

    let cache = PackageCache {
        debian: debian.join().unwrap_or(None),
        arch: arch.join().unwrap_or(None),
        redhat: redhat.join().unwrap_or(None),
        void: void.join().unwrap_or(None),
        gentoo: gentoo.join().unwrap_or(None),
        alpine: alpine.join().unwrap_or(None),
        flatpak: flatpak.join().unwrap_or(None),
        suse: suse.join().unwrap_or(None),
        netbsd: netbsd.join().unwrap_or(None),
        termux: termux.join().unwrap_or(None),
        timestamp: SystemTime::now(),
    };

    let _ = std::fs::write(&cp, serde_json::to_string(&cache).unwrap());

    format_package_string(&cache)
}

fn format_package_string(cache: &PackageCache) -> String {
    let mut parts = Vec::new();
    
    if let Some(count) = cache.debian {
        parts.push(format!("{} (deb  )", count));
    }
    if let Some(count) = cache.arch {
        parts.push(format!("{} (arch 󰣇 )", count));
    }
    if let Some(count) = cache.redhat {
        parts.push(format!("{} (dnf  )", count));
    }
    if let Some(count) = cache.void {
        parts.push(format!("{} (void  )", count));
    }
    if let Some(count) = cache.gentoo {
        parts.push(format!("{} (gent 󰣨 )", count));
    }
    if let Some(count) = cache.alpine {
        parts.push(format!("{} (alpine  )", count));
    }
    if let Some(count) = cache.flatpak {
        parts.push(format!("{} (flatpak 󰏖 )", count));
    }
    if let Some(count) = cache.suse {
        parts.push(format!("{} (suse  )", count));
    }
    if let Some(count) = cache.termux {
        parts.push(format!("{} (termux  )", count));
    }

    if parts.is_empty() {
        "none found".to_string()
    } else {
        format!("{}", parts.join(", "))
    }
}

pub fn clear_cache() {
    let _ = std::fs::remove_file(cache_path());
}

pub fn getform() -> String {
    get_installed_packages_parallel()
}
