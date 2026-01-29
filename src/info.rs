use std::path::Path;
use sysinfo::System;

use crate::cache::Cache;

pub struct SystemInfo {
    pub user: String,
    pub hostname: String,
    pub os: String,
    pub host: String,
    pub kernel: String,
    pub uptime: String,
    pub packages: String,
    pub shell: String,
    pub resolution: String,
    pub de: String,
    pub wm: String,
    pub wm_theme: String,
    pub terminal: String,
    pub cpu: String,
    pub gpu: String,
    pub memory: String,
    pub disk: String,
    pub local_ip: String,
}

impl SystemInfo {
    pub fn gather() -> Self {
        let sys = System::new_with_specifics(
            sysinfo::RefreshKind::new()
                .with_cpu(sysinfo::CpuRefreshKind::new())
                .with_memory(sysinfo::MemoryRefreshKind::everything()),
        );

        let mut cache = Cache::load();

        // Cached per boot cycle (never change until reboot)
        let os = cache.get_or_insert("os", get_os_version);
        let host = cache.get_or_insert("host", get_host_model);
        let kernel = cache.get_or_insert("kernel", get_kernel);
        let cpu = cache.get_or_insert("cpu", || get_cpu(&sys));
        let gpu = cache.get_or_insert("gpu", || get_gpu(&sys));
        let de = cache.get_or_insert("de", get_de);
        let wm = cache.get_or_insert("wm", get_wm);
        let shell = cache.get_or_insert("shell", get_shell);
        let resolution = cache.get_or_insert("resolution", get_resolution);
        let wm_theme = cache.get_or_insert("wm_theme", get_wm_theme);
        let packages = cache.get_or_insert("packages", get_packages);

        cache.save();

        // Always fresh (changes every run)
        Self {
            user: get_user(),
            hostname: get_hostname(),
            os,
            host,
            kernel,
            uptime: get_uptime(),
            packages,
            shell,
            resolution,
            de,
            wm,
            wm_theme,
            terminal: get_terminal(),
            cpu,
            gpu,
            memory: get_memory(&sys),
            disk: get_disk(),
            local_ip: get_local_ip(),
        }
    }

    pub fn title(&self) -> String {
        format!("{}@{}", self.user, self.hostname)
    }

    pub fn to_json(&self) -> String {
        let mut map = serde_json::Map::new();
        for (k, v) in self.fields() {
            map.insert(k.to_string(), serde_json::Value::String(v.to_string()));
        }
        serde_json::to_string_pretty(&map).unwrap_or_else(|_| "{}".into())
    }

    pub fn fields(&self) -> Vec<(&str, &str)> {
        vec![
            ("OS", &self.os),
            ("Host", &self.host),
            ("Kernel", &self.kernel),
            ("Uptime", &self.uptime),
            ("Packages", &self.packages),
            ("Shell", &self.shell),
            ("Resolution", &self.resolution),
            ("DE", &self.de),
            ("WM", &self.wm),
            ("WM Theme", &self.wm_theme),
            ("Terminal", &self.terminal),
            ("CPU", &self.cpu),
            ("GPU", &self.gpu),
            ("Memory", &self.memory),
            ("Disk (/)", &self.disk),
            ("Local IP", &self.local_ip),
        ]
    }
}

fn get_user() -> String {
    std::env::var("USER").unwrap_or_else(|_| "user".into())
}

fn get_hostname() -> String {
    System::host_name().unwrap_or_else(|| "Unknown".into())
}

fn get_os_version() -> String {
    let os = System::long_os_version().unwrap_or_else(|| "Unknown".into());
    let arch = System::cpu_arch().unwrap_or_default();
    if arch.is_empty() {
        os
    } else {
        format!("{} {}", os, arch)
    }
}

#[cfg(target_os = "macos")]
fn sysctl_string(name: &[u8]) -> Option<String> {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    extern "C" {
        fn sysctlbyname(
            name: *const c_char,
            oldp: *mut u8,
            oldlenp: *mut usize,
            newp: *const u8,
            newlen: usize,
        ) -> i32;
    }

    let mut buf = [0u8; 256];
    let mut len = buf.len();

    unsafe {
        if sysctlbyname(
            name.as_ptr() as *const c_char,
            buf.as_mut_ptr(),
            &mut len,
            std::ptr::null(),
            0,
        ) == 0
        {
            Some(
                CStr::from_ptr(buf.as_ptr() as *const c_char)
                    .to_string_lossy()
                    .into_owned(),
            )
        } else {
            None
        }
    }
}

fn get_host_model() -> String {
    #[cfg(target_os = "macos")]
    {
        sysctl_string(b"hw.model\0").unwrap_or_else(|| "Unknown".into())
    }
    #[cfg(not(target_os = "macos"))]
    {
        std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "Unknown".into())
    }
}

fn get_kernel() -> String {
    System::kernel_version().unwrap_or_else(|| "Unknown".into())
}

fn get_uptime() -> String {
    let secs = System::uptime();
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
    }
    if hours > 0 {
        parts.push(format!("{} hour{}", hours, if hours == 1 { "" } else { "s" }));
    }
    if mins > 0 {
        parts.push(format!("{} min{}", mins, if mins == 1 { "" } else { "s" }));
    }
    if parts.is_empty() {
        format!("{} secs", secs)
    } else {
        parts.join(", ")
    }
}

fn get_packages() -> String {
    #[cfg(target_os = "macos")]
    {
        let cellar = Path::new("/usr/local/Cellar");
        let cellar_alt = Path::new("/opt/homebrew/Cellar");
        let dir = if cellar_alt.exists() {
            cellar_alt
        } else if cellar.exists() {
            cellar
        } else {
            return "Unknown".into();
        };

        match std::fs::read_dir(dir) {
            Ok(entries) => {
                let count = entries.filter(|e| e.is_ok()).count();
                format!("{} (brew)", count)
            }
            Err(_) => "Unknown".into(),
        }
    }
    #[cfg(target_os = "linux")]
    {
        get_packages_linux()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        "Unknown".into()
    }
}

#[cfg(target_os = "linux")]
fn get_packages_linux() -> String {
    let mut parts = Vec::new();

    // dpkg (Debian/Ubuntu)
    if let Ok(entries) = std::fs::read_dir("/var/lib/dpkg/info") {
        let count = entries
            .filter(|e| {
                e.as_ref()
                    .ok()
                    .and_then(|e| e.file_name().to_str().map(|s| s.ends_with(".list")))
                    .unwrap_or(false)
            })
            .count();
        if count > 0 {
            parts.push(format!("{} (dpkg)", count));
        }
    }

    // pacman (Arch)
    if let Ok(entries) = std::fs::read_dir("/var/lib/pacman/local") {
        let count = entries.filter(|e| e.is_ok()).count().saturating_sub(1); // minus ALPM_DB_VERSION
        if count > 0 {
            parts.push(format!("{} (pacman)", count));
        }
    }

    // rpm (Fedora/RHEL)
    if Path::new("/var/lib/rpm").exists() {
        if let Ok(out) = std::process::Command::new("rpm").args(["-qa", "--last"]).output() {
            if out.status.success() {
                let count = String::from_utf8_lossy(&out.stdout).lines().count();
                if count > 0 {
                    parts.push(format!("{} (rpm)", count));
                }
            }
        }
    }

    // flatpak
    if let Ok(entries) = std::fs::read_dir("/var/lib/flatpak/app") {
        let count = entries.filter(|e| e.is_ok()).count();
        if count > 0 {
            parts.push(format!("{} (flatpak)", count));
        }
    }

    // snap
    if let Ok(entries) = std::fs::read_dir("/snap") {
        let count = entries
            .filter(|e| {
                e.as_ref()
                    .ok()
                    .and_then(|e| {
                        let name = e.file_name();
                        let s = name.to_str()?;
                        Some(s != "bin" && s != "README" && !s.starts_with('.'))
                    })
                    .unwrap_or(false)
            })
            .count();
        if count > 0 {
            parts.push(format!("{} (snap)", count));
        }
    }

    if parts.is_empty() {
        "Unknown".into()
    } else {
        parts.join(", ")
    }
}

fn get_shell() -> String {
    std::env::var("SHELL")
        .ok()
        .and_then(|s| {
            let name = s.rsplit('/').next().map(String::from)?;
            let version = std::env::var("ZSH_VERSION")
                .or_else(|_| std::env::var("BASH_VERSION"))
                .ok();
            match version {
                Some(v) => Some(format!("{} {}", name, v)),
                None => Some(name),
            }
        })
        .unwrap_or_else(|| "Unknown".into())
}

fn get_resolution() -> String {
    #[cfg(target_os = "macos")]
    {
        type CGDirectDisplayID = u32;

        extern "C" {
            fn CGDisplayPixelsWide(display: CGDirectDisplayID) -> usize;
            fn CGDisplayPixelsHigh(display: CGDirectDisplayID) -> usize;
            fn CGGetOnlineDisplayList(
                max: u32,
                displays: *mut CGDirectDisplayID,
                count: *mut u32,
            ) -> i32;
        }

        let mut displays = [0u32; 16];
        let mut count: u32 = 0;

        unsafe {
            if CGGetOnlineDisplayList(16, displays.as_mut_ptr(), &mut count) != 0 {
                return "Unknown".into();
            }

            let mut resolutions = Vec::new();
            for i in 0..count as usize {
                let w = CGDisplayPixelsWide(displays[i]);
                let h = CGDisplayPixelsHigh(displays[i]);
                if w > 0 && h > 0 {
                    resolutions.push(format!("{}x{}", w, h));
                }
            }

            if resolutions.is_empty() {
                "Unknown".into()
            } else {
                resolutions.join(", ")
            }
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        "Unknown".into()
    }
}

fn get_de() -> String {
    #[cfg(target_os = "macos")]
    {
        "Aqua".into()
    }
    #[cfg(not(target_os = "macos"))]
    {
        std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("DESKTOP_SESSION"))
            .unwrap_or_else(|_| "Unknown".into())
    }
}

fn get_wm() -> String {
    #[cfg(target_os = "macos")]
    {
        "Quartz Compositor".into()
    }
    #[cfg(not(target_os = "macos"))]
    {
        std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "Unknown".into())
    }
}

fn get_wm_theme() -> String {
    #[cfg(target_os = "macos")]
    {
        let style = std::process::Command::new("defaults")
            .args(["read", "-g", "AppleInterfaceStyle"])
            .output()
            .ok()
            .and_then(|out| {
                if out.status.success() {
                    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "Light".into());

        let color = std::process::Command::new("defaults")
            .args(["read", "-g", "AppleAccentColor"])
            .output()
            .ok()
            .and_then(|out| {
                if out.status.success() {
                    let val = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    Some(
                        match val.as_str() {
                            "-2" => "Purple",
                            "-1" => "Pink",
                            "0" => "Red",
                            "1" => "Orange",
                            "2" => "Yellow",
                            "3" => "Green",
                            "4" | "" => "Blue",
                            "5" => "Purple",
                            "6" => "Graphite",
                            _ => "Blue",
                        }
                        .to_string(),
                    )
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "Blue".into());

        format!("{} ({})", color, style)
    }
    #[cfg(target_os = "linux")]
    {
        // Read GTK theme from settings.ini
        if let Ok(home) = std::env::var("HOME") {
            let path = format!("{}/.config/gtk-3.0/settings.ini", home);
            if let Ok(content) = std::fs::read_to_string(&path) {
                for line in content.lines() {
                    if let Some(rest) = line.strip_prefix("gtk-theme-name=") {
                        return rest.trim().to_string();
                    }
                }
            }
        }
        "Unknown".into()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        "Unknown".into()
    }
}

fn get_terminal() -> String {
    std::env::var("TERM_PROGRAM").unwrap_or_else(|_| {
        std::env::var("TERM").unwrap_or_else(|_| "Unknown".into())
    })
}

fn get_cpu(sys: &System) -> String {
    let brand = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown".into());
    let cores = sys.physical_core_count();
    match cores {
        Some(n) => format!("{} ({})", brand, n),
        None => brand,
    }
}

fn get_gpu(sys: &System) -> String {
    #[cfg(target_os = "macos")]
    {
        // On Apple Silicon, GPU is integrated into the SoC
        sys.cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Unknown".into())
    }
    #[cfg(target_os = "linux")]
    {
        let _ = sys;
        // Try reading from /proc/driver/nvidia/version or lspci
        if let Ok(content) = std::fs::read_to_string("/proc/driver/nvidia/version") {
            if let Some(line) = content.lines().next() {
                return line.to_string();
            }
        }
        // Fallback: parse lspci for VGA
        if let Ok(out) = std::process::Command::new("lspci").output() {
            if out.status.success() {
                let text = String::from_utf8_lossy(&out.stdout);
                for line in text.lines() {
                    if line.contains("VGA") || line.contains("3D") || line.contains("Display") {
                        if let Some(desc) = line.split(": ").nth(1) {
                            return desc.to_string();
                        }
                    }
                }
            }
        }
        "Unknown".into()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        let _ = sys;
        "Unknown".into()
    }
}

fn get_memory(sys: &System) -> String {
    let used = sys.used_memory() / 1024 / 1024;
    let total = sys.total_memory() / 1024 / 1024;
    format!("{}MiB / {}MiB", used, total)
}

fn get_disk() -> String {
    #[cfg(unix)]
    {
        use std::mem::MaybeUninit;

        extern "C" {
            fn statfs(path: *const i8, buf: *mut libc_statfs) -> i32;
        }

        #[cfg(target_os = "macos")]
        #[repr(C)]
        struct libc_statfs {
            f_bsize: u32,
            f_iosize: i32,
            f_blocks: u64,
            f_bfree: u64,
            f_bavail: u64,
            f_files: u64,
            f_ffree: u64,
            f_fsid: [i32; 2],
            f_owner: u32,
            f_type: u32,
            f_flags: u32,
            f_fssubtype: u32,
            f_fstypename: [u8; 16],
            f_mntonname: [u8; 1024],
            f_mntfromname: [u8; 1024],
            f_flags_ext: u32,
            f_reserved: [u32; 7],
        }

        #[cfg(target_os = "linux")]
        #[repr(C)]
        struct libc_statfs {
            f_type: i64,
            f_bsize: i64,
            f_blocks: u64,
            f_bfree: u64,
            f_bavail: u64,
            f_files: u64,
            f_ffree: u64,
            f_fsid: [i32; 2],
            f_namelen: i64,
            f_frsize: i64,
            f_flags: i64,
            f_spare: [i64; 4],
        }

        let path = b"/\0";
        let mut buf = MaybeUninit::<libc_statfs>::uninit();

        unsafe {
            if statfs(path.as_ptr() as *const i8, buf.as_mut_ptr()) == 0 {
                let buf = buf.assume_init();
                let block_size = buf.f_bsize as u64;
                let total = (buf.f_blocks * block_size) / 1024 / 1024 / 1024;
                let available = (buf.f_bavail * block_size) / 1024 / 1024 / 1024;
                let used = total - available;
                let percent = if total > 0 { (used * 100) / total } else { 0 };
                return format!("{}GiB / {}GiB ({}%)", used, total, percent);
            }
        }
        "Unknown".into()
    }
    #[cfg(not(unix))]
    {
        "Unknown".into()
    }
}

fn get_local_ip() -> String {
    use sysinfo::Networks;

    let networks = Networks::new_with_refreshed_list();
    for (name, data) in &networks {
        if name == "lo" || name == "lo0" || name.starts_with("veth") || name.starts_with("docker") {
            continue;
        }
        for ip in data.ip_networks() {
            let addr = ip.addr;
            if addr.is_loopback() {
                continue;
            }
            if let std::net::IpAddr::V4(v4) = addr {
                if !v4.is_link_local() {
                    return v4.to_string();
                }
            }
        }
    }
    "Unknown".into()
}
