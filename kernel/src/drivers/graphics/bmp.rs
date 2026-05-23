pub struct Bmp<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
    pixel_offset: usize,
    row_size: usize,
    bottom_up: bool,
    bpp: u16,
}

impl<'a> Bmp<'a> {
    pub fn new(data: &'a [u8]) -> Option<Self> {
        if data.len() < 54 {
            return None;
        }

        if &data[0..2] != b"BM" {
            return None;
        }

        let pixel_offset = read_u32(data, 10) as usize;
        let width = read_i32(data, 18);
        let height = read_i32(data, 22);
        let bpp = read_u16(data, 28);
        let compression = read_u32(data, 30);

        if width <= 0 || height == 0 {
            return None;
        }

        if bpp != 24 && bpp != 32 {
            return None;
        }

        if compression != 0 {
            return None;
        }

        let width = width as usize;
        let bottom_up = height > 0;
        let height = height.abs() as usize;

        let row_size = match bpp {
            24 => (width * 3 + 3) & !3,
            32 => width * 4,
            _ => return None,
        };

        Some(Self {
            data,
            width,
            height,
            pixel_offset,
            row_size,
            bottom_up,
            bpp,
        })
    }

    pub fn draw(
        &self,
        fb: &mut crate::drivers::graphics::framebuffer::Framebuffer,
        x: usize,
        y: usize,
    ) {
        let bytes_per_pixel = (self.bpp / 8) as usize;

        for img_y in 0..self.height {
            let src_y = if self.bottom_up {
                self.height - 1 - img_y
            } else {
                img_y
            };

            for img_x in 0..self.width {
                let offset = self.pixel_offset + src_y * self.row_size + img_x * bytes_per_pixel;

                if offset + 2 >= self.data.len() {
                    continue;
                }

                let b = self.data[offset] as u32;
                let g = self.data[offset + 1] as u32;
                let r = self.data[offset + 2] as u32;

                if self.bpp == 32 {
                    if offset + 3 >= self.data.len() {
                        continue;
                    }

                    let a = self.data[offset + 3];

                    if a == 0 {
                        continue;
                    }
                }

                let color = (r << 16) | (g << 8) | b;

                fb.put_pixel(x + img_x, y + img_y, color);
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn bits_per_pixel(&self) -> u16 {
        self.bpp
    }
}

fn read_u16(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

fn read_u32(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

fn read_i32(data: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}
