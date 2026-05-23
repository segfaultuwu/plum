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
