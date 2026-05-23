pub struct VGA {
    fg: u8,
    bg: u8,
    row: usize,
    col: usize,
}

impl VGA {
    pub fn new(fg: u8, bg: u8) -> Self {
        Self {
            fg,
            bg,
            row: 0,
            col: 0,
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn write_line(&mut self, s: &str) {
        self.write_string(s);
        self.new_line();
    }

    pub fn update_cursor(&mut self, row: usize, col: usize) {
        let pos = (row * 80 + col) as u16;

        unsafe {
            crate::drivers::serial::outb(0x3D4, 0x0F);
            crate::drivers::serial::outb(0x3D5, (pos & 0xFF) as u8);

            crate::drivers::serial::outb(0x3D4, 0x0E);
            crate::drivers::serial::outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= 80 {
                    self.new_line();
                }

                let offset = (self.row * 80 + self.col) * 2;
                let vga_buffer = 0xb8000 as *mut u8;

                unsafe {
                    *vga_buffer.add(offset) = byte;
                    *vga_buffer.add(offset + 1) = (self.bg << 4) | self.fg;
                }

                self.col += 1;
                self.update_cursor(self.row, self.col);
            }
        }
    }

    fn new_line(&mut self) {
        self.col = 0;

        if self.row >= 24 {
            self.scroll();
        } else {
            self.row += 1;
        }
    }

    pub fn clear(&mut self) {
        let vga_buffer = 0xb8000 as *mut u8;

        for i in 0..(80 * 25) {
            unsafe {
                *vga_buffer.add(i * 2) = b' ';
                *vga_buffer.add(i * 2 + 1) = (self.bg << 4) | self.fg;
            }
        }

        self.row = 0;
        self.col = 0;
    }

    pub fn set_colors(&mut self, fg: u8, bg: u8) {
        self.fg = fg;
        self.bg = bg;
    }

    pub fn scroll(&mut self) {
        let vga_buffer = 0xb8000 as *mut u8;

        for i in 0..(80 * 24) {
            unsafe {
                *vga_buffer.add(i * 2) = *vga_buffer.add((i + 80) * 2);
                *vga_buffer.add(i * 2 + 1) = *vga_buffer.add((i + 80) * 2 + 1);
            }
        }

        for i in (80 * 24)..(80 * 25) {
            unsafe {
                *vga_buffer.add(i * 2) = b' ';
                *vga_buffer.add(i * 2 + 1) = (self.bg << 4) | self.fg;
            }
        }
    }
}
