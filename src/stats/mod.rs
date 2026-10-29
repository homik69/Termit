#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

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