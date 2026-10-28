use std::process::Command;

pub fn get_cpu_ticks() -> (u64, u64) {
    let cpuinfo = std::fs::read_to_string("/proc/stat").unwrap();
    let usage = cpuinfo.lines().next().unwrap().split_whitespace().skip(1);
    let usage_to_vec: Vec<u64> = usage.map(|x| x.parse::<u64>().unwrap()).collect();
    return (usage_to_vec[3] + usage_to_vec[4], usage_to_vec.iter().sum());
}

pub fn get_cpu_name() -> String {
    let cpu = std::fs::read_to_string("/proc/cpuinfo").unwrap();

    cpu.lines()
        .find(|line| line.starts_with("model name"))
        .and_then(|line| line.split(':').nth(1))
        .map(|name| name.trim().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string())
}

pub fn get_gpu_name() -> String {
    let output = Command::new("lspci").output().ok();

    let stdout = match output {
        Some(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        None => return "Unknown GPU".to_string(),
    };

    stdout
        .lines()
        .find(|line| line.contains("VGA compatible controller") || line.contains("3D controller"))
        .and_then(|line| line.split(':').nth(2))
        .map(|name| name.trim().to_string())
        .unwrap_or_else(|| "Unknown GPU".to_string())
}

pub fn get_ram_info() -> (u32, u32) {
    let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap();
    let mut total_kb = 0;
    let mut available_kb = 0;
    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            total_kb = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
        }

        if line.starts_with("MemAvailable:") {
            available_kb = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
        }
    }

    return (((total_kb - available_kb) / 1024), (total_kb / 1024));
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortMode {
    Ram,
    Cpu,
}

pub struct Process {
    pub pid: u32,
    pub name: String,
    pub ram_mb: u32,
    pub cpu_ticks: u64,
}

pub fn get_processes(sort_by: SortMode) -> Vec<Process> {
    let mut processes = Vec::new();

    let entries = match std::fs::read_dir("/proc") {
        Ok(dir) => dir,
        Err(_) => return processes,
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        let pid = match name_str.parse::<u32>() {
            Ok(comm) => comm,
            Err(_) => continue,
        };

        let comm_path = format!("/proc/{}/comm", pid);
        let proc_name = match std::fs::read_to_string(&comm_path) {
            Ok(comm) => comm.trim().to_string(),
            Err(_) => continue,
        };

        let statm_path = format!("/proc/{}/statm", pid);
        let ram_mb = match std::fs::read_to_string(&statm_path) {
            Ok(statm) => statm.split_whitespace().nth(1).and_then(|s| s.parse::<u64>().ok()).map(|pages| ((pages * 4) / 1024) as u32).unwrap_or(0),
            Err(_) => 0,
        };

        let stat_path = format!("/proc/{}/stat", pid);
        let cpu_ticks = match std::fs::read_to_string(&stat_path) {
            Ok(content) => {
                if let Some(rest) = content.rfind(')').map(|idx| &content[idx + 2..]) {
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    let utime = parts.get(11).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                    let stime = parts.get(12).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                    utime + stime
                } else {
                    0
                }
            }
            Err(_) => 0,
        };

        processes.push(Process {
            pid,
            name: proc_name,
            ram_mb: ram_mb,
            cpu_ticks
        });
    }

    // processes.sort_by_key(|p| p.pid);

    match sort_by {
        SortMode::Ram => processes.sort_by(|a, b| b.ram_mb.cmp(&a.ram_mb)),
        SortMode::Cpu => processes.sort_by(|a, b| b.cpu_ticks.cmp(&a.cpu_ticks)),
    }

    processes
}
