use super::canvas::Canvas;
use crate::info::SystemInfo;

/// Build a compact one-line status string from system info.
/// Format: "macOS 15.5 · M3 Pro · 28/36 GiB · 250/500 GiB · 33d up"
pub fn build_line(info: &SystemInfo) -> String {
    let mut parts = Vec::new();

    // OS — strip architecture suffix
    let os = info
        .os
        .split(" arm")
        .next()
        .unwrap_or(&info.os)
        .split(" x86")
        .next()
        .unwrap_or(&info.os);
    parts.push(os.to_string());

    // CPU — strip core count and "Apple " prefix
    let cpu = info.cpu.split(" (").next().unwrap_or(&info.cpu);
    let cpu = cpu.strip_prefix("Apple ").unwrap_or(cpu);
    parts.push(cpu.to_string());

    // Memory — convert MiB to GiB compact
    if let Some(mem) = compact_memory(&info.memory) {
        parts.push(mem);
    }

    // Disk/SSD — compact
    if let Some(disk) = compact_disk(&info.disk) {
        parts.push(disk);
    }

    // Uptime — compact
    parts.push(compact_uptime(&info.uptime));

    parts.join(" · ")
}

fn compact_memory(mem: &str) -> Option<String> {
    let halves: Vec<&str> = mem.split('/').collect();
    if halves.len() != 2 {
        return None;
    }
    let used_mib: u64 = halves[0].trim().strip_suffix("MiB")?.trim().parse().ok()?;
    let total_mib: u64 = halves[1].trim().strip_suffix("MiB")?.trim().parse().ok()?;
    Some(format!("{}/{} GiB", used_mib / 1024, total_mib / 1024))
}

fn compact_disk(disk: &str) -> Option<String> {
    // Input like "250GiB / 500GiB (50%)"
    let without_pct = disk.split('(').next().unwrap_or(disk).trim();
    let halves: Vec<&str> = without_pct.split('/').collect();
    if halves.len() != 2 {
        return None;
    }
    let used = halves[0].trim().strip_suffix("GiB")?.trim();
    let total = halves[1].trim().strip_suffix("GiB")?.trim();
    Some(format!("{}/{} GiB SSD", used, total))
}

fn compact_uptime(uptime: &str) -> String {
    let first = uptime.split(',').next().unwrap_or(uptime).trim();
    let compact = first
        .replace(" days", "d")
        .replace(" day", "d")
        .replace(" hours", "h")
        .replace(" hour", "h")
        .replace(" mins", "m")
        .replace(" min", "m")
        .replace(" secs", "s");
    format!("{} up", compact)
}

/// Draw the status bar at a given row.
/// `progress` controls the typing animation (0.0–1.0).
pub fn draw(canvas: &mut Canvas, text: &str, row: u16, color: (u8, u8, u8), progress: f32) {
    let total_chars = text.chars().count();
    let max_chars = ((total_chars as f32) * progress) as usize;
    let visible: String = text.chars().take(max_chars).collect();
    canvas.put_str(2, row, &visible, color, None);
}
