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
    timestamp: SystemTime,
}

fn get_installed_packages_parallel() -> String {
    let cache_path = "/tmp/rfetch_packages.json";
    if let Ok(data) = std::fs::read_to_string(cache_path) {
        if let Ok(cache) = serde_json::from_str::<PackageCache>(&data) {
            if cache.timestamp.elapsed().unwrap_or_default() < Duration::from_secs(3600) {
                return format_package_string(&cache);
            }
        }
    }

    let debian = thread::spawn(|| {
        if Command::new("dpkg").arg("--version").output().is_ok() {
            Command::new("dpkg")
                .arg("--get-selections")
                .output()
                .ok()
                .and_then(|o| {
                    let count = String::from_utf8_lossy(&o.stdout).lines().count();
                    if count > 0 { Some(count) } else { None }
                })
        } else {
            None
        }
    });

    let arch = thread::spawn(|| {
        if Command::new("pacman").arg("--version").output().is_ok() {
            Command::new("pacman")
                .arg("-Q")
                .output()
                .ok()
                .and_then(|o| {
                    let count = String::from_utf8_lossy(&o.stdout).lines().count();
                    if count > 0 { Some(count) } else { None }
                })
        } else {
            None
        }
    });

    let redhat = thread::spawn(|| {
        if Command::new("dnf").arg("--version").output().is_ok() {
            Command::new("dnf")
                .arg("list")
                .arg("--installed")
                .output()
                .ok()
                .and_then(|o| {
                    let count = String::from_utf8_lossy(&o.stdout).lines().count();
                    if count > 0 { Some(count) } else { None }
                })
        } else {
            None
        }
    });

    let void = thread::spawn(|| {
        if Command::new("xbps-query").arg("--version").output().is_ok() {
            Command::new("xbps-query")
                .arg("-l")
                .output()
                .ok()
                .and_then(|o| {
                    let count = String::from_utf8_lossy(&o.stdout).lines().count();
                    if count > 0 { Some(count) } else { None }
                })
        } else {
            None
        }
    });

    let gentoo = thread::spawn(|| {
        if Command::new("emerge").arg("--version").output().is_ok() {
            Command::new("qlist")
                .arg("-Iv")
                .output()
                .ok()
                .and_then(|o| {
                    let count = String::from_utf8_lossy(&o.stdout).lines().count();
                    if count > 0 { Some(count) } else { None }
                })
        } else {
            None
        }
    });

    let cache = PackageCache {
        debian: debian.join().unwrap_or(None),
        arch: arch.join().unwrap_or(None),
        redhat: redhat.join().unwrap_or(None),
        void: void.join().unwrap_or(None),
        gentoo: gentoo.join().unwrap_or(None),
        timestamp: SystemTime::now(),
    };

    let _ = std::fs::write(cache_path, serde_json::to_string(&cache).unwrap());

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
        parts.push(format!("{} (dnf  )", count));
    }
    if let Some(count) = cache.void {
        parts.push(format!("{} (void  )", count));
    }
    if let Some(count) = cache.gentoo {
        parts.push(format!("{} (gent  )", count));
    }

    if parts.is_empty() {
        "|   packages: none found (are you on Windows?)".to_string()
    } else {
        format!("  packages: {}", parts.join(", "))
    }
}

pub fn getform() -> String {
    get_installed_packages_parallel()
}
