use core::arch::asm;

pub unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value);
    }
}

pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;

    unsafe {
        asm!("in al, dx", out("al") value, in("dx") port);
    }

    value
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
