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
