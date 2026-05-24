use core::fmt;

use x86_64::instructions::tlb::flush;

use crate::drivers::graphics::{framebuffer::Framebuffer, psf::PSF};
use crate::flush;

pub struct Terminal<'a> {
    framebuffer: &'a mut Framebuffer<'a>,
    font: &'static PSF,
    cursor_x: usize,
    cursor_y: usize,
    fg: u32,
    default_fg: u32,
}

impl<'a> Terminal<'a> {
    pub fn new(framebuffer: &'a mut Framebuffer<'a>, font: &'static PSF, fg: u32) -> Self {
        Self {
            framebuffer,
            font,
            cursor_x: 10,
            cursor_y: 40,
            fg,
            default_fg: fg,
        }
    }

    pub fn write_string(&mut self, s: &str) {
        let bytes = s.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] == 0x1B && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
                i += 2;

                let mut code: u16 = 0;

                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    code = code * 10 + (bytes[i] - b'0') as u16;
                    i += 1;
                }

                if i < bytes.len() && bytes[i] == b'm' {
                    match code {
                        0 => self.fg = self.default_fg,
                        30 => self.fg = 0x11111B,
                        31 => self.fg = 0xF38BA8,
                        32 => self.fg = 0xA6E3A1,
                        33 => self.fg = 0xF9E2AF,
                        34 => self.fg = 0x89B4FA,
                        35 => self.fg = 0xCBA6F7,
                        36 => self.fg = 0x94E2D5,
                        37 => self.fg = 0xCDD6F4,

                        90 => self.fg = 0x6C7086,
                        91 => self.fg = 0xF38BA8,
                        92 => self.fg = 0xA6E3A1,
                        93 => self.fg = 0xF9E2AF,
                        94 => self.fg = 0x89B4FA,
                        95 => self.fg = 0xCBA6F7,
                        96 => self.fg = 0x94E2D5,
                        97 => self.fg = 0xCDD6F4,

                        _ => {}
                    }

                    i += 1;
                    continue;
                }
            }

            match bytes[i] {
                b'\n' => {
                    self.cursor_x = 0;
                    self.cursor_y += self.font.height;
                }
                b'\r' => {
                    self.cursor_x = 0;
                }
                b'\x08' => {
                    if self.cursor_x >= self.font.width {
                        self.cursor_x -= self.font.width;

                        for yy in 0..self.font.height {
                            for xx in 0..self.font.width {
                                self.framebuffer.put_pixel(
                                    self.cursor_x + xx,
                                    self.cursor_y + yy,
                                    0x1E1E2E,
                                );
                            }
                        }
                    }
                }
                b'\t' => {
                    let spaces = 4 - (self.cursor_x / self.font.width) % 4;
                    self.cursor_x += spaces * self.font.width;
                }
                byte => {
                    self.font.draw_char(
                        self.framebuffer,
                        byte as char,
                        self.cursor_x,
                        self.cursor_y,
                        self.fg,
                    );

                    self.cursor_x += self.font.width;
                }
            }

            i += 1;
        }
    }
    pub fn clear_current_line(&mut self) {
        let y = self.cursor_y;

        for row in 0..self.font.height {
            for col in 0..self.framebuffer.width {
                self.framebuffer.put_pixel(col, y + row, 0x1E1E2E);
            }
        }

        self.cursor_x = 0;
        flush!();
    }

    pub fn set_color(&mut self, fg: u32) {
        self.fg = fg;
        flush!();
    }

    pub fn clear_screen(&mut self, color: u32) {
        self.framebuffer.clear(color);
        self.cursor_x = 0;
        self.cursor_y = 0;
        flush!();
    }

    pub fn reset(&mut self) {
        self.fg = self.default_fg;
        self.clear_screen(0x1E1E2E);
        self.cursor_x = 0;
        self.cursor_y = 0;
        flush!();
    }
}

impl<'a> fmt::Write for Terminal<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

fn ansi_color(code: u8) -> Option<u32> {
    match code {
        30 => Some(0x000000),
        31 => Some(0xF38BA8),
        32 => Some(0xA6E3A1),
        33 => Some(0xF9E2AF),
        34 => Some(0x89B4FA),
        35 => Some(0xF5C2E7),
        36 => Some(0x94E2D5),
        37 => Some(0xCDD6F4),
        _ => None,
    }
}
