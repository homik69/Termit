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

impl SortMode {
    pub fn toggle(&mut self) {
        *self = match self {
            SortMode::Ram => SortMode::Cpu,
            SortMode::Cpu => SortMode::Ram,
        };
    }
}

pub struct Process {
    pub pid: u32,
    pub name: String,
    pub ram_mb: u32,
    pub cpu_ticks: u64,
}