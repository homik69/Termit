mod stats;
mod terminal;
mod ui;

use std::io::Write;

use terminal::RawTermit;
use crate::stats::{get_cpu_ticks, get_ram_info};

fn main() {
    println!("Welcome to Termit");

    let _termit = RawTermit::new().unwrap();
    
    let (mut prev_idle, mut prev_total) = get_cpu_ticks();

    let cpu_name = stats::get_cpu_name();
    let gpu_name = stats::get_gpu_name();

    let mut sort_mode = stats::SortMode::Ram;

    let mut scroll_offset: usize = 0;

    let mut prev_size = (0usize, 0usize);

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

    loop {

        let (term_w, term_h) = terminal::get_termit_size();

        if (term_w, term_h) != prev_size {
            print!("\x1b[2J");
            prev_size = (term_w, term_h);
        }

        print!("\x1b[H");

        let screen = ui::Rect {
            x: 1,
            y: 2,
            width: term_w,
            height: term_h.saturating_sub(2),
        };

        let (top_area, bottom_rect) = screen.split_vertical(60);

        let (left_rect, right_rect) = top_area.split_horizontal(50);

        let (used_ram, total_ram) = get_ram_info();

        let (curr_idle, curr_total) = get_cpu_ticks();
        let total_diff = curr_total - prev_total;
        let idle_diff = curr_idle - prev_idle;

        let cpu_percent = if total_diff > 0 {
            ((total_diff - idle_diff) * 100) / total_diff
        } else {
            0
        };
        ui::create_window("CPU & RAM", left_rect);

        let cpu_text = format!("{}%", cpu_percent);
        ui::draw_row("CPU Usage:", &cpu_text, left_rect, 0);

        let ram_text = format!("{} / {} MB", used_ram, total_ram);
        ui::draw_row("RAM Usage:", &ram_text, left_rect, 1);

        ui::draw_row(
            "Sort:",
            if sort_mode == stats::SortMode::Ram {
                "RAM [Tab]"
            } else {
                "CPU [Tab]"
            },
            left_rect,
            2,
        );
        ui::draw_row("Exit key:", "Q", left_rect, 3);

        ui::fill_empty_rows(left_rect, 4);
        ui::draw_footer(left_rect);

        ui::create_window("Hardware", right_rect);

        ui::draw_row("CPU Model:", &cpu_name, right_rect, 0);

        ui::draw_row("GPU Model:", &clean_gpu_name, right_rect, 1);

        ui::fill_empty_rows(right_rect, 2);
        ui::draw_footer(right_rect);

        let proc_list = stats::get_processes(sort_mode);

        let max_rows = bottom_rect.height.saturating_sub(2);
        let window_title = match sort_mode {
            stats::SortMode::Ram => format!(
                "Processes (RAM) [{}-{}/{}]",
                scroll_offset + 1,
                (scroll_offset + max_rows).min(proc_list.len()),
                proc_list.len()
            ),
            stats::SortMode::Cpu => format!(
                "Processes (CPU) [{}-{}/{}]",
                scroll_offset + 1,
                (scroll_offset + max_rows).min(proc_list.len()),
                proc_list.len()
            ),
        };
        ui::create_window(&window_title, bottom_rect);

        let max_scroll = proc_list.len().saturating_sub(max_rows);
        scroll_offset = scroll_offset.min(max_scroll);

        let mut rendered_count = 0;

        for (idx, p) in proc_list
            .iter()
            .skip(scroll_offset)
            .take(max_rows)
            .enumerate()
        {
            let label = format!("{}:", p.name);
            let value = match sort_mode {
                stats::SortMode::Ram => format!("{} MB (PID {})", p.ram_mb, p.pid),
                stats::SortMode::Cpu => format!("{} ticks (PID {})", p.cpu_ticks, p.pid),
            };
            ui::draw_row(&label, &value, bottom_rect, idx);
            rendered_count = idx + 1;
        }

        ui::fill_empty_rows(bottom_rect, rendered_count);
        ui::draw_footer(bottom_rect);
        prev_idle = curr_idle;
        prev_total = curr_total;

        std::io::stdout().flush().expect("prank");

        if let Some(key) = _termit.read_key(500) {
            match key {
                terminal::Key::Quit => break,
                terminal::Key::Tab => {
                    sort_mode.toggle();
                }
                terminal::Key::Up => {
                    scroll_offset = scroll_offset.saturating_sub(1);
                }
                terminal::Key::Down => {
                    let max_scroll = proc_list.len().saturating_sub(max_rows);
                    if scroll_offset < max_scroll {
                        scroll_offset += 1;
                    }
                }
                _ => {}
            }
        }
    }
}
