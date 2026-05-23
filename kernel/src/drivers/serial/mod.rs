use core::arch::asm;

pub unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value);
}

pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", out("al") value, in("dx") port);
    value
}

pub fn init() {
    unsafe {
        outb(0x3F8 + 1, 0x00);
        outb(0x3F8 + 3, 0x80);
        outb(0x3F8 + 0, 0x03);
        outb(0x3F8 + 1, 0x00);
        outb(0x3F8 + 3, 0x03);
        outb(0x3F8 + 2, 0xC7);
        outb(0x3F8 + 4, 0x0B);
    }
}

pub fn write_byte(byte: u8) {
    unsafe {
        while (inb(0x3F8 + 5) & 0x20) == 0 {}
        outb(0x3F8, byte);
    }
}

pub fn write(s: &str) {
    for b in s.bytes() {
        write_byte(b);
    }
}
