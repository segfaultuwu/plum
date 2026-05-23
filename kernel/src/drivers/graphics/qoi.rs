pub struct Qoi<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
    pixel_offset: usize,
    channels: u8,
}

#[derive(Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl<'a> Qoi<'a> {
    pub fn new(data: &'a [u8]) -> Option<Self> {
        if data.len() < 14 + 8 {
            return None;
        }

        if &data[0..4] != b"qoif" {
            return None;
        }

        let width = read_u32_be(data, 4) as usize;
        let height = read_u32_be(data, 8) as usize;
        let channels = data[12];

        if width == 0 || height == 0 {
            return None;
        }

        if channels != 3 && channels != 4 {
            return None;
        }

        Some(Self {
            data,
            width,
            height,
            pixel_offset: 14,
            channels,
        })
    }

    pub fn draw(
        &self,
        fb: &mut crate::drivers::graphics::framebuffer::Framebuffer,
        x: usize,
        y: usize,
    ) {
        let mut index = [Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }; 64];

        let mut px = Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };

        let mut data_pos = self.pixel_offset;
        let mut run = 0usize;

        for py in 0..self.height {
            for px_x in 0..self.width {
                if run > 0 {
                    run -= 1;
                } else {
                    if data_pos >= self.data.len() {
                        return;
                    }

                    let b1 = self.data[data_pos];
                    data_pos += 1;

                    match b1 {
                        0xFE => {
                            if data_pos + 2 >= self.data.len() {
                                return;
                            }

                            px.r = self.data[data_pos];
                            px.g = self.data[data_pos + 1];
                            px.b = self.data[data_pos + 2];
                            data_pos += 3;
                        }

                        0xFF => {
                            if data_pos + 3 >= self.data.len() {
                                return;
                            }

                            px.r = self.data[data_pos];
                            px.g = self.data[data_pos + 1];
                            px.b = self.data[data_pos + 2];
                            px.a = self.data[data_pos + 3];
                            data_pos += 4;
                        }

                        _ => {
                            let tag = b1 & 0b1100_0000;

                            match tag {
                                0b0000_0000 => {
                                    px = index[b1 as usize];
                                }

                                0b0100_0000 => {
                                    let dr = ((b1 >> 4) & 0x03) as i8 - 2;
                                    let dg = ((b1 >> 2) & 0x03) as i8 - 2;
                                    let db = (b1 & 0x03) as i8 - 2;

                                    px.r = px.r.wrapping_add(dr as u8);
                                    px.g = px.g.wrapping_add(dg as u8);
                                    px.b = px.b.wrapping_add(db as u8);
                                }

                                0b1000_0000 => {
                                    if data_pos >= self.data.len() {
                                        return;
                                    }

                                    let b2 = self.data[data_pos];
                                    data_pos += 1;

                                    let dg = (b1 & 0x3F) as i8 - 32;
                                    let dr_dg = ((b2 >> 4) & 0x0F) as i8 - 8;
                                    let db_dg = (b2 & 0x0F) as i8 - 8;

                                    let dr = dg + dr_dg;
                                    let db = dg + db_dg;

                                    px.r = px.r.wrapping_add(dr as u8);
                                    px.g = px.g.wrapping_add(dg as u8);
                                    px.b = px.b.wrapping_add(db as u8);
                                }

                                0b1100_0000 => {
                                    run = (b1 & 0x3F) as usize;
                                }

                                _ => {}
                            }
                        }
                    }

                    let idx = qoi_hash(px);
                    index[idx] = px;
                }

                if px.a == 255 {
                    fb.put_pixel(x + px_x, y + py, rgb(px.r, px.g, px.b));
                } else if px.a != 0 {
                    let dst = fb.get_pixel(x + px_x, y + py);

                    let dr = ((dst >> 16) & 0xff) as u32;
                    let dg = ((dst >> 8) & 0xff) as u32;
                    let db = (dst & 0xff) as u32;

                    let a = px.a as u32;
                    let sr = px.r as u32;
                    let sg = px.g as u32;
                    let sb = px.b as u32;

                    let r = (sr * a + dr * (255 - a)) / 255;
                    let g = (sg * a + dg * (255 - a)) / 255;
                    let b = (sb * a + db * (255 - a)) / 255;

                    fb.put_pixel(x + px_x, y + py, rgb(r as u8, g as u8, b as u8));
                }
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn channels(&self) -> u8 {
        self.channels
    }
}

fn qoi_hash(px: Pixel) -> usize {
    (px.r as usize * 3 + px.g as usize * 5 + px.b as usize * 7 + px.a as usize * 11) % 64
}

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | b as u32
}

fn read_u32_be(data: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}
