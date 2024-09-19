//! ANSI helpers for generating escape sequences.

use crate::play::Point;

pub const RED: u8 = 1;
pub const GREEN: u8 = 2;
pub const YELLOW: u8 = 3;
pub const BLUE: u8 = 4;
pub const MAGENTA: u8 = 5;
pub const CYAN: u8 = 6;
pub const WHITE: u8 = 7;
pub const GRAY: u8 = 8;

const CSI: &str = "\x1b[";

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
    format!("{CSI}38;5;{color}m")
}

pub fn reset_color() -> String {
    format!("{CSI}0m")
}
