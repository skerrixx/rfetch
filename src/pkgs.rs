use serde::{Deserialize, Serialize};
use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime};

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

#[derive(Serialize, Deserialize)]
struct GpuCache {
    gpu: String,
    timestamp: SystemTime,
}

fn count_packages(cmd: &str, args: &[&str]) -> Option<usize> {
    Command::new(cmd).args(args).output().ok().and_then(|o| {
        if !o.status.success() {
            return None;
        }
        let count = String::from_utf8_lossy(&o.stdout).lines().count();
        if count > 0 { Some(count) } else { None }
    })
}

fn binary_exists(cmd: &str) -> bool {
    if cmd.contains('/') {
        return std::fs::exists(cmd).unwrap_or(false);
    }
    let path = match std::env::var("PATH") {
        Ok(p) if !p.trim().is_empty() => p,
        _ => return true, // PATH unknown: fall back to trying the spawn (old behavior)
    };
    for dir in path.split(':') {
        if dir.is_empty() {
            continue;
        }
        if std::fs::exists(format!("{}/{}", dir, cmd)).unwrap_or(false) {
            return true;
        }
    }
    false
}

fn count_if_present(cmd: &str, args: &[&str]) -> Option<usize> {
    // skip the fork/exec entirely for managers that aren't installed.
    // output-identical: a missing binary fails the spawn and yields None anyway.
    if !binary_exists(cmd) {
        return None;
    }
    count_packages(cmd, args)
}

fn cache_path() -> String {
    match std::env::var("TMPDIR") {
        Ok(tmpdir) if !tmpdir.trim().is_empty() => format!("{}/rfetch_packages.json", tmpdir),
        _ => "/tmp/rfetch_packages.json".to_string(),
    }
}

fn read_fresh_cache() -> Option<PackageCache> {
    let data = std::fs::read_to_string(cache_path()).ok()?;
    let cache: PackageCache = serde_json::from_str(&data).ok()?;
    if cache.timestamp.elapsed().unwrap_or_default() < Duration::from_secs(3600) {
        Some(cache)
    } else {
        None
    }
}

fn gpu_cache_path() -> String {
    match std::env::var("TMPDIR") {
        Ok(tmpdir) if !tmpdir.trim().is_empty() => format!("{}/rfetch_gpu.json", tmpdir),
        _ => "/tmp/rfetch_gpu.json".to_string(),
    }
}

pub fn cached_gpu() -> Option<String> {
    // gpu string lives in its own cache file so it stays warm even when
    // "packages" is hidden. previously both shared one file written only by
    // getform(), so hiding packages starved this fast path: cached_gpu()
    // missed forever and every run paid a full drm probe (~20ms).
    let data = std::fs::read_to_string(gpu_cache_path()).ok()?;
    let cache: GpuCache = serde_json::from_str(&data).ok()?;
    if cache.timestamp.elapsed().unwrap_or_default() < Duration::from_secs(3600) {
        Some(cache.gpu)
    } else {
        None
    }
}

pub fn fetch_gpu() -> String {
    if let Some(g) = cached_gpu() {
        return g;
    }
    let g = crate::basic::gpu();
    let cache = GpuCache {
        gpu: g.clone(),
        timestamp: SystemTime::now(),
    };
    let _ = std::fs::write(gpu_cache_path(), serde_json::to_string(&cache).unwrap());
    g
}

fn get_installed_packages_parallel() -> String {
    if let Some(cache) = read_fresh_cache() {
        return format_package_string(&cache);
    }

    let debian = thread::spawn(|| count_if_present("dpkg", &["--get-selections"]));
    let arch = thread::spawn(|| count_if_present("pacman", &["-Q"]));
    let redhat = thread::spawn(|| count_if_present("dnf", &["list", "--installed"]));
    let alpine = thread::spawn(|| count_if_present("apk", &["info"]));
    let void = thread::spawn(|| count_if_present("xbps-query", &["-l"]));
    let flatpak = thread::spawn(|| count_if_present("flatpak", &["list"]));
    let gentoo = thread::spawn(|| count_if_present("qlist", &["-Iv"]));
    let suse = thread::spawn(|| count_if_present("zypper", &["se", "-i"]));
    let netbsd = thread::spawn(|| count_if_present("pkg_info", &["-q"]));
    let termux = thread::spawn(|| count_if_present("dpkg-query", &["-f", "${Status}\n", "--show"]));

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

    let _ = std::fs::write(cache_path(), serde_json::to_string(&cache).unwrap());

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
    let _ = std::fs::remove_file(gpu_cache_path());
}

pub fn getform() -> String {
    get_installed_packages_parallel()
}
