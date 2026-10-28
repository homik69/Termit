pub const RESET: &str = "\x1b[0m";
pub const BORDER_COLOR: &str = "\x1b[38;2;120;110;160m";
pub const TITLE_COLOR: &str = "\x1b[38;2;215;180;255m";
pub const LABEL_COLOR: &str = "\x1b[38;2;170;130;240m";
pub const VALUE_COLOR: &str = "\x1b[38;2;240;225;255m";

pub fn create_window(title: &str, rect: Rect) {
    let remaining_dashes = rect.width.saturating_sub(title.len() + 6);
    let line = "─".repeat(remaining_dashes);

    print!(
        "\x1b[{};{}H{}╭─{} {}{}{} {}{}─╮{}",
        rect.y, rect.x, BORDER_COLOR, RESET, TITLE_COLOR, title, RESET, BORDER_COLOR, line, RESET
    );
}

pub fn draw_row(label: &str, value: &str, rect: Rect, row_idx: usize) {
    let visible_len = label.len() + value.len();
    let padding = rect.width.saturating_sub(5).saturating_sub(visible_len);
    let spaces = " ".repeat(padding);

    print!(
        "\x1b[{};{}H{}│{} {}{}{} {}{}{}{} {}│{}",
        rect.y + 1 + row_idx,
        rect.x,
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

pub fn draw_footer(rect: Rect) {
    let dashes = "─".repeat(rect.width.saturating_sub(2));
    print!(
        "\x1b[{};{}H{}╰{}╯{}",
        rect.y + rect.height - 1,
        rect.x,
        BORDER_COLOR,
        dashes,
        RESET
    );
}

// fn cursor_movement(row: usize, col: usize) {
//     print!("\x1b[{row};{col}H")
// }

pub fn fill_empty_rows(rect: Rect, start_row: usize) {
    let total_content_rows = rect.height.saturating_sub(2);
    let inner_spaces = " ".repeat(rect.width.saturating_sub(2));

    for row_idx in start_row..total_content_rows {
        print!(
            "\x1b[{};{}H{}│{}{}{}│{}",
            rect.y + 1 + row_idx,
            rect.x,
            BORDER_COLOR,
            RESET,
            inner_spaces,
            BORDER_COLOR,
            RESET
        );
    }
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}
