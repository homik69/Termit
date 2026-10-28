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

    let cpu_name = stats::get_cpu_name();
    let gpu_name = stats::get_gpu_name();

    loop {
        print!("\x1b[2J");

        let (term_w, term_h) = terminal::get_termit_size();

        let main_height = term_h.saturating_sub(2);
        let half_width = term_w / 2;

        let left_rect = ui::Rect {
            x: 1,
            y: 2,
            width: half_width,
            height: main_height,
        };

        let right_rect = ui::Rect {
            x: 1 + half_width,
            y: 2,
            width: term_w.saturating_sub(half_width),
            height: main_height,
        };

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
        ui::create_window("one", left_rect);
        // print!("CPU Usage: {}%\x1b[K\r\n", cpu_percent);

        let cpu_text = format!("{}%", cpu_percent);
        ui::draw_row("CPU Usage:", &cpu_text, left_rect, 0);

        let ram_text = format!("{} / {} MB", used_ram, total_ram);
        ui::draw_row("RAM Usage:", &ram_text, left_rect, 1);

        ui::fill_empty_rows(left_rect, 2);
        ui::draw_row("Button to quit:", "Q", left_rect, 3);

        ui::draw_footer(left_rect);
        ui::create_window("two", right_rect);

        ui::draw_row("CPU Model:", &cpu_name, right_rect, 0);

        let clean_gpu_name = gpu_name
            .replace("Advanced Micro Devices, Inc. [AMD/ATI]", "")
            .replace("Corporation", "")
            .replace("rev c0", "")
            .replace("Navi 48", "")
            .replace("[", "")
            .replace("]", "")
            .replace("()", "")
            .trim()
            .to_string();

        ui::draw_row("GPU Model:", &clean_gpu_name, right_rect, 1);

        ui::fill_empty_rows(right_rect, 2);
        ui::draw_footer(right_rect);
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
