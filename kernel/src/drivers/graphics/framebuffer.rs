use bootloader_api::info::{FrameBuffer, PixelFormat};

pub struct Framebuffer<'a> {
    buffer: &'a mut [u8],
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub bytes_per_pixel: usize,
    pub pixel_format: PixelFormat,
}

impl<'a> Framebuffer<'a> {
    pub fn from_bootloader(fb: &'a mut FrameBuffer) -> Self {
        let info = fb.info();

        Self {
            buffer: fb.buffer_mut(),
            width: info.width,
            height: info.height,
            stride: info.stride,
            bytes_per_pixel: info.bytes_per_pixel,
            pixel_format: info.pixel_format,
        }
    }

    pub fn clear(&mut self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.put_pixel(x, y, color);
            }
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let offset = (y * self.stride + x) * self.bytes_per_pixel;

        let r = ((color >> 16) & 0xff) as u8;
        let g = ((color >> 8) & 0xff) as u8;
        let b = (color & 0xff) as u8;

        match self.pixel_format {
            PixelFormat::Rgb => {
                self.buffer[offset] = r;
                self.buffer[offset + 1] = g;
                self.buffer[offset + 2] = b;
            }
            PixelFormat::Bgr => {
                self.buffer[offset] = b;
                self.buffer[offset + 1] = g;
                self.buffer[offset + 2] = r;
            }
            _ => {}
        }
    }
}
