use std::process::Command;

pub fn get_cpu_name() -> String {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg("machdep.cpu.brand_string")
        .output()
        .ok();

    output
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "Apple Silicon / Intel".to_string())
}

pub fn get_gpu_name() -> String {
    let output = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output()
        .ok();

    let stdout = match output {
        Some(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        None => return "Apple GPU".to_string(),
    };

    stdout
        .lines()
        .find(|line| line.trim().starts_with("Chipset Model:"))
        .and_then(|line| line.split(':').nth(1))
        .map(|name| name.trim().to_string())
        .unwrap_or_else(|| "Apple GPU".to_string())
}

pub fn get_ram_info() -> (u32, u32) {
    let total_bytes = Command::new("sysctl")
        .arg("-n")
        .arg("hw.memsize")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().parse::<u64>().ok())
        .unwrap_or(0);
    let total_mb = (total_bytes / 1024 / 1024) as u32;

    let vm_output = Command::new("vm_stat")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    let mut used_pages: u64 = 0;
    let page_size: u64 = 4096;

    for line in vm_output.lines() {
        if line.starts_with("Pages active:") 
            || line.starts_with("Pages wired down:") 
            || line.starts_with("Pages occupied by compressor:") 
        {
            if let Some(val_str) = line.split(':').nth(1) {
                let cleaned = val_str.trim().trim_end_matches('.');
                if let Ok(pages) = cleaned.parse::<u64>() {
                    used_pages += pages;
                }
            }
        }
    }

    let used_mb = ((used_pages * page_size) / 1024 / 1024) as u32;
    (used_mb, total_mb)
}

pub fn get_cpu_ticks() -> (u64, u64) {
    (0,100)
}

use super::{Process, SortMode};

pub fn get_processes(sort_by: SortMode) -> Vec<Process> {
    let output = Command::new("ps")
        .args(["-axo", "pid,rss,time,comm"])
        .output()
        .ok();

    let mut processes = Vec::new();
    let stdout = match output {
        Some(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        None => return processes,
    };

    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }

        let pid = match parts[0].parse::<u32>() {
            Ok(p) => p,
            Err(_) => continue,
        };

        let ram_mb = parts[1].parse::<u32>().unwrap_or(0) / 1024;

        let name = parts[3..].join(" ");
        let clean_name = name.rsplit('/').next().unwrap_or(&name).to_string();

        processes.push(Process {
            pid,
            name: clean_name,
            ram_mb,
            cpu_ticks: 0,
        });
    }

    match sort_by {
        SortMode::Ram => processes.sort_by(|a, b| b.ram_mb.cmp(&a.ram_mb)),
        SortMode::Cpu => processes.sort_by(|a, b| b.cpu_ticks.cmp(&a.cpu_ticks)),
    }

    processes
}