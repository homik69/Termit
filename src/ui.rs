pub const RESET: &str = "\x1b[0m";
pub const BORDER_COLOR: &str = "\x1b[38;2;120;110;160m";
pub const TITLE_COLOR: &str = "\x1b[38;2;215;180;255m";
pub const LABEL_COLOR: &str = "\x1b[38;2;170;130;240m";
pub const VALUE_COLOR: &str = "\x1b[38;2;240;225;255m";

pub fn create_window(title: &str, width: usize) {
    let remaining_dashes = width.saturating_sub(title.len() + 6);
    let line = "─".repeat(remaining_dashes);

    print!(
        "{}╭─{} {}{}{} {}{}─╮{}\x1b[K\r\n",
        BORDER_COLOR, RESET, TITLE_COLOR, title, RESET, BORDER_COLOR, line, RESET
    );
}

pub fn draw_row(label: &str, value: &str, width: usize) {
    let visible_len = label.len() + value.len();
    let padding = width.saturating_sub(5).saturating_sub(visible_len);
    let spaces = " ".repeat(padding);

    print!(
        "{}│{} {}{}{} {}{}{}{} {}│{}\x1b[K\r\n",
        BORDER_COLOR,
        RESET,
        LABEL_COLOR,
        label,
        RESET,
        VALUE_COLOR,
        value,
        RESET,
        spaces,
        BORDER_COLOR,
        RESET
    );
}

pub fn draw_footer(width: usize) {
    let dashes = "─".repeat(width.saturating_sub(2));
    print!("{}╰{}╯{}\x1b[K\r\n", BORDER_COLOR, dashes, RESET);
}
