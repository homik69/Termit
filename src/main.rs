mod stats;
mod terminal;
mod ui;

use std::io::Read;
use std::io::Write;

use stats::{get_cpu_ticks, get_ram_info};
use terminal::RawTermit;
fn main() {
    println!("Welcome to Termit");

    let _termit = RawTermit::new().unwrap();

    let mut poll = libc::pollfd {
        fd: libc::STDIN_FILENO,
        events: libc::POLLIN,
        revents: 0,
    };

    let (mut prev_idle, mut prev_total) = get_cpu_ticks();

    loop {
        print!("\x1b[HTermit by homik (q to quit)\x1b[K\r\n");

        // let title = ui::color_rgb(189, 147, 249, "Termit by homik");

        ui::create_window("Termit by homik", 50);

        let (used_ram, total_ram) = get_ram_info();
        // print!("Ram: {} out of {}\x1b[K\r\n", used_ram, total_ram);

        let (curr_idle, curr_total) = get_cpu_ticks();
        let total_diff = curr_total - prev_total;
        let idle_diff = curr_idle - prev_idle;

        let cpu_percent = if total_diff > 0 {
            ((total_diff - idle_diff) * 100) / total_diff
        } else {
            0
        };

        // print!("CPU Usage: {}%\x1b[K\r\n", cpu_percent);

        let cpu_name = stats::get_cpu_name();
        ui::draw_row("CPU Model:", &cpu_name, 50);

        let cpu_text = format!("{}%", cpu_percent);
        ui::draw_row("CPU Usage:", &cpu_text, 50);

        ui::draw_row(" ", " ", 50);

        let ram_text = format!("{} / {} MB", used_ram, total_ram);
        ui::draw_row("RAM Usage:", &ram_text, 50);

        ui::draw_row(" ", " ", 50);

        let gpu_name = stats::get_gpu_name();

        let clean_gpu_name = gpu_name.trim()
            .replace("Advanced Micro Devices, Inc. [AMD/ATI]", "")
            .replace("Corporation", "")
            .replace("rev c0", "")
            .replace("Navi 48", "")
            .replace("[", "")
            .replace("]", "")
            .replace("()", "")
            .trim().to_string();

        ui::draw_row("GPU Model:", &clean_gpu_name, 50);

        ui::draw_footer(50);
        prev_idle = curr_idle;
        prev_total = curr_total;

        std::io::stdout().flush().expect("prank");

        let ready = unsafe { libc::poll(&mut poll, 1, 500) };
        let mut buf = [0u8; 1];

        if ready > 0 {
            std::io::stdin().read(&mut buf).expect("prank");
        }

        if buf[0] == b'q' {
            break;
        }
    }
}
