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

    let alpine = thread::spawn(|| {
            if Command::new("apk").arg("--version").output().is_ok() {
                Command::new("apk")
                    .arg("info")
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
	let flatpak = thread::spawn(|| {
            if Command::new("flatpak").arg("--version").output().is_ok() {
                Command::new("flatpak")
                    .arg("list")
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
    let suse = thread::spawn(|| {
        if Command::new("zypper").arg("--version").output().is_ok() {
            Command::new("zypper")
                .arg("se")
                .arg("-i")
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
        alpine: alpine.join().unwrap_or(None),
        flatpak: flatpak.join().unwrap_or(None),
        suse: suse.join().unwrap_or(None),
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
    if let Some(count) = cache.alpine {
        parts.push(format!("{} (alpine  )", count));
    }
    if let Some(count) = cache.flatpak {
        parts.push(format!("{} (flatpak  )", count));
    }
    if let Some(count) = cache.suse {
        parts.push(format!("{} (suse  )", count));
    }

    if parts.is_empty() {
        "|   packages: none found".to_string()
    } else {
        format!("  packages: {}", parts.join(", "))
    }
}

pub fn clear_cache() {
    let cache_path = "/tmp/rfetch_packages.json";
    let _ = std::fs::remove_file(cache_path);
}

pub fn getform() -> String {
    get_installed_packages_parallel()
}
