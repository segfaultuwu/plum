use alloc::{format, string::String};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Colors {
    Text,
    Surface0,
    Blue,
    Green,
    Red,
}

impl Colors {
    pub fn to_vga(self) -> u8 {
        match self {
            Self::Text => 0xF,
            Self::Surface0 => 0x0,
            Self::Blue => 0x9,
            Self::Green => 0xA,
            Self::Red => 0xC,
        }
    }
    pub fn to_rgb(self) -> u32 {
        match self {
            Self::Text => 0xCDD6F4,
            Self::Surface0 => 0x313244,
            Self::Blue => 0x89B4FA,
            Self::Green => 0xA6E3A1,
            Self::Red => 0xF38BA8,
        }
    }
}

pub enum AnsiColor {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,

    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightWhite = 97,
}

pub fn ansi_wrap(color: AnsiColor, text: &str) -> String {
    format!("\x1B[{}m{}\x1B[0m", color as u8, text)
}
