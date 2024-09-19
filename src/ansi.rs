//! ANSI escape sequences.

use crate::bounce::Point;

pub const CSI: &str = "\x1b[";

pub fn clear_screen() -> String {
    format!("{CSI}2J")
}

pub fn set_cursor(p: &Point) -> String {
    format!("{CSI}{};{}H", p.row + 1, p.col + 1)
}

pub fn set_cursor_to(row: u16, col: u16) -> String {
    format!("{CSI}{};{}H", row + 1, col + 1)
}

pub fn set_color(color: u8) -> String {
    format!("{CSI}{color}m")
}

pub fn reset_color() -> String {
    format!("{CSI}0m")
}
