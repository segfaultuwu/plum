use alloc::vec;
use alloc::vec::Vec;
use bootloader_api::info::{FrameBuffer, PixelFormat};

pub struct Framebuffer<'a> {
    front: &'a mut [u8],
    back: Vec<u8>,

    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub bytes_per_pixel: usize,

    pixel_format: PixelFormat,
}

impl<'a> Framebuffer<'a> {
    pub fn from_bootloader(fb: &'a mut FrameBuffer) -> Self {
        let info = fb.info();
        let front = fb.buffer_mut();
        let back = vec![0; info.byte_len];

        Self {
            front,
            back,
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

    pub fn swap_buffers(&mut self) {
        self.front.copy_from_slice(&self.back);
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let offset = (y * self.stride + x) * self.bytes_per_pixel;

        if offset + self.bytes_per_pixel > self.back.len() {
            return;
        }

        let r = ((color >> 16) & 0xff) as u8;
        let g = ((color >> 8) & 0xff) as u8;
        let b = (color & 0xff) as u8;

        match self.pixel_format {
            PixelFormat::Rgb => {
                self.back[offset] = r;
                self.back[offset + 1] = g;
                self.back[offset + 2] = b;
            }
            PixelFormat::Bgr => {
                self.back[offset] = b;
                self.back[offset + 1] = g;
                self.back[offset + 2] = r;
            }
            PixelFormat::U8 => {
                self.back[offset] = r;
            }
            _ => {}
        }
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for yy in y..y + height {
            for xx in x..x + width {
                self.put_pixel(xx, yy, color);
            }
        }
    }

    pub fn draw_horizontal_line(&mut self, x: usize, y: usize, width: usize, color: u32) {
        for xx in x..x + width {
            self.put_pixel(xx, y, color);
        }
    }

    pub fn draw_vertical_line(&mut self, x: usize, y: usize, height: usize, color: u32) {
        for yy in y..y + height {
            self.put_pixel(x, yy, color);
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        if x >= self.width || y >= self.height {
            return 0;
        }

        let offset = (y * self.stride + x) * self.bytes_per_pixel;

        if offset + 2 >= self.back.len() {
            return 0;
        }

        match self.pixel_format {
            PixelFormat::Rgb => {
                let r = self.back[offset] as u32;
                let g = self.back[offset + 1] as u32;
                let b = self.back[offset + 2] as u32;

                (r << 16) | (g << 8) | b
            }
            PixelFormat::Bgr => {
                let b = self.back[offset] as u32;
                let g = self.back[offset + 1] as u32;
                let r = self.back[offset + 2] as u32;

                (r << 16) | (g << 8) | b
            }
            PixelFormat::U8 => {
                let v = self.back[offset] as u32;
                (v << 16) | (v << 8) | v
            }
            _ => 0,
        }
    }
}
