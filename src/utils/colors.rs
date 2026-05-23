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
}
