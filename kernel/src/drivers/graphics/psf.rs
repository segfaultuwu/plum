pub struct PSF {
    pub width: usize,
    pub height: usize,
    pub glyphs: &'static [u8],
}

impl PSF {
    pub fn new(width: usize, height: usize, glyphs: &'static [u8]) -> Self {
        Self {
            width,
            height,
            glyphs,
        }
    }

    pub fn draw_char(
        &self,
        framebuffer: &mut crate::drivers::graphics::framebuffer::Framebuffer,
        c: char,
        x: usize,
        y: usize,
        color: u32,
    ) {
        let glyph_index = c as usize;
        let glyph_size = self.width * self.height / 8;
        let glyph_data = &self.glyphs[glyph_index * glyph_size..(glyph_index + 1) * glyph_size];

        for row in 0..self.height {
            for col in 0..self.width {
                let byte_index = (row * self.width + col) / 8;
                let bit_index = 7 - (col % 8);
                if (glyph_data[byte_index] >> bit_index) & 1 == 1 {
                    framebuffer.put_pixel(x + col, y + row, color);
                }
            }
        }
    }

    pub fn draw_string(
        &self,
        framebuffer: &mut crate::drivers::graphics::framebuffer::Framebuffer,
        s: &str,
        x: usize,
        y: usize,
        color: u32,
    ) {
        for (i, c) in s.chars().enumerate() {
            self.draw_char(framebuffer, c, x + i * self.width, y, color);
        }
    }
}
