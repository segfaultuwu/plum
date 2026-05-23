use crate::drivers;

pub mod colors;

pub fn vga_write_usize(vga: &mut drivers::graphics::vga::VGA, mut n: usize) {
    let mut buf = [0u8; 20];
    let mut i = buf.len();

    if n == 0 {
        vga.write_string("0");
        return;
    }

    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }

    let s = core::str::from_utf8(&buf[i..]).unwrap();
    vga.write_string(s);
}

pub fn serial_write_usize(mut n: usize) {
    let mut buf = [0u8; 20];
    let mut i = buf.len();

    if n == 0 {
        drivers::serial::write("0");
        return;
    }

    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }

    let s = core::str::from_utf8(&buf[i..]).unwrap();
    drivers::serial::write(s);
}
