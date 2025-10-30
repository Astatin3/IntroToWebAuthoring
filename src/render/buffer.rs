use crate::render::rand;

pub struct ImgBuffer {
    pub data: Vec<u8>,
    width: usize,
    height: usize,
}

impl ImgBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        ImgBuffer {
            data: vec![127; width * height * 4],
            width,
            height,
        }
    }

    pub fn randomize(&mut self, rand: &mut rand::Rnd) {
        for pixel in self.data.chunks_mut(4) {
            pixel[0] = rand.next_i32() as u8;
            pixel[1] = pixel[0];
            pixel[2] = pixel[0];
            pixel[3] = 255;
        }
    }

    pub fn overlay_bitmap(&mut self, other: &Bitmap, xoffset: usize, yoffset: usize) {
        let length = self.data.len();
        for y in 0..other.height {
            for x in 0..other.width {
                // Calculate the offset for the current pixel in the buffer
                // This uses bitshift instead of multiplication x4 for performance
                let offset = ((y + yoffset) * self.width + (x + xoffset)) << 2;
                if offset >= length {
                    return;
                }
                let color = other.data[y * other.width + x];
                self.data[offset] = color;
                self.data[offset + 1] = color;
                self.data[offset + 2] = color;
                self.data[offset + 3] = 255;
            }
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.data = vec![0; width * height * 4];
        self.width = width;
        self.height = height;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl ImgBuffer {
    pub fn set_pixel(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        let index = (y * self.width + x) * 4;
        self.data[index] = color.0;
        self.data[index + 1] = color.1;
        self.data[index + 2] = color.2;
        self.data[index + 3] = 255;
    }
}

pub struct Bitmap {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn from_data(data: Vec<u8>, width: usize, height: usize) -> Self {
        // assert!(data.len() == width * height, "Invalid data length!");z
        Self {
            data,
            width,
            height,
        }
    }

    pub fn overlay(&mut self, other: &Bitmap, xoffset: usize, yoffset: usize) {
        for y in 0..other.height {
            for x in 0..other.width {
                self.data[(y + yoffset as usize) * self.width + (x + xoffset as usize)] =
                    other.data[y * other.width + x];
            }
        }
    }

    /// Scale using nearest-neighbor (faster but lower quality)
    pub fn scale(&self, scale_x: f32, scale_y: f32) -> Bitmap {
        let new_width = (self.width as f32 * scale_x).round() as usize;
        let new_height = (self.height as f32 * scale_y).round() as usize;

        let mut new_data = vec![0u8; new_width * new_height];

        for y in 0..new_height {
            let src_y = ((y as f32) / scale_y) as usize;
            let src_y = src_y.min(self.height - 1);

            for x in 0..new_width {
                let src_x = ((x as f32) / scale_x) as usize;
                let src_x = src_x.min(self.width - 1);

                new_data[y * new_width + x] = self.data[src_y * self.width + src_x];
            }
        }

        Bitmap {
            data: new_data,
            width: new_width,
            height: new_height,
        }
    }
}
