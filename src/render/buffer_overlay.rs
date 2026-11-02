use crate::render::buffer::{Bitmap, ImgBuffer};

impl ImgBuffer {
    pub fn overlay_bitmap_fg_bg(
        &mut self,
        other: &Bitmap,
        xoffset: usize,
        yoffset: usize,
        foreground_color: (u8, u8, u8, u8),
        background_color: (u8, u8, u8, u8),
    ) {
        let length = self.data.len();
        for y in 0..other.height {
            for x in 0..other.width {
                let offset = ((y + yoffset) * self.width + (x + xoffset)) << 2;
                if offset >= length {
                    return;
                }
                let weight = other.data[y * other.width + x];

                let (mut pixel_r, mut pixel_g, mut pixel_b, mut pixel_a) = (
                    self.data[offset],
                    self.data[offset + 1],
                    self.data[offset + 2],
                    self.data[offset + 3],
                );

                (pixel_r, pixel_g, pixel_b, pixel_a) = Self::blend_bitmap_pixel_fg_bg(
                    (pixel_r, pixel_b, pixel_g, pixel_a),
                    weight,
                    foreground_color,
                    background_color,
                );

                self.data[offset] = pixel_r;
                self.data[offset + 1] = pixel_g;
                self.data[offset + 2] = pixel_b;
                self.data[offset + 3] = pixel_a;

                //  = color;
                // self.data[offset + 1] = color;
                // self.data[offset + 2] = color;
                // self.data[offset + 3] = 255;
            }
        }
    }

    pub fn overlay_bitmap_fg(
        &mut self,
        other: &Bitmap,
        xoffset: usize,
        yoffset: usize,
        foreground_color: (u8, u8, u8),
    ) {
        let length = self.data.len();
        for y in 0..other.height {
            for x in 0..other.width {
                let offset = ((y + yoffset) * self.width + (x + xoffset)) << 2;
                if offset >= length {
                    return;
                }
                let weight = other.data[y * other.width + x];

                let (mut pixel_r, mut pixel_g, mut pixel_b, mut pixel_a) = (
                    self.data[offset],
                    self.data[offset + 1],
                    self.data[offset + 2],
                    self.data[offset + 3],
                );

                (pixel_r, pixel_g, pixel_b, pixel_a) = Self::blend_bitmap_pixel_fg(
                    (pixel_r, pixel_b, pixel_g, pixel_a),
                    weight,
                    foreground_color,
                );

                self.data[offset] = pixel_r;
                self.data[offset + 1] = pixel_b;
                self.data[offset + 2] = pixel_g;
                self.data[offset + 3] = pixel_a;

                //  = color;
                // self.data[offset + 1] = color;
                // self.data[offset + 2] = color;
                // self.data[offset + 3] = 255;
            }
        }
    }

    pub fn blend_bitmap_pixel_fg_bg(
        original: (u8, u8, u8, u8),
        bitmap_weight: u8,
        fg_color: (u8, u8, u8, u8),
        bg_color: (u8, u8, u8, u8),
    ) -> (u8, u8, u8, u8) {
        // Step 1: Interpolate between bg_color and fg_color based on bitmap_weight

        let w = bitmap_weight as u32;
        let inv_w = 255u32 - w;

        // Blend RGB channels using (x * 257) >> 16 approximation for /255
        let br = (((bg_color.0 as u32 * inv_w + fg_color.0 as u32 * w) * 257) >> 16) as u8;
        let bg = (((bg_color.1 as u32 * inv_w + fg_color.1 as u32 * w) * 257) >> 16) as u8;
        let bb = (((bg_color.2 as u32 * inv_w + fg_color.2 as u32 * w) * 257) >> 16) as u8;
        let ba = (((bg_color.3 as u32 * inv_w + fg_color.3 as u32 * w) * 257) >> 16) as u8;

        // Step 2: Alpha blend the interpolated color over the original pixel

        if ba == 0 {
            // Fully transparent, return original
            return original;
        }

        if ba == 255 {
            // Fully opaque, return blended color
            return (br, bg, bb, 255);
        }

        let a = ba as u32;
        let inv_a = 255u32 - a;

        let r = (((original.0 as u32 * inv_a + br as u32 * a) * 257) >> 16) as u8;
        let g = (((original.1 as u32 * inv_a + bg as u32 * a) * 257) >> 16) as u8;
        let b = (((original.2 as u32 * inv_a + bb as u32 * a) * 257) >> 16) as u8;

        // Alpha channel: blend using "over" operation
        let final_a = (((original.3 as u32 * inv_a) * 257) >> 16) + a;
        let final_a = final_a.min(255) as u8;

        (r, g, b, final_a)
    }

    pub fn blend_bitmap_pixel_fg(
        original: (u8, u8, u8, u8),
        bitmap_weight: u8,
        fg_color: (u8, u8, u8),
    ) -> (u8, u8, u8, u8) {
        if bitmap_weight == 0 {
            // Fully transparent, return original
            return original;
        }

        if bitmap_weight == 255 {
            // Fully opaque, return foreground color
            return (fg_color.0, fg_color.1, fg_color.2, 255);
        }

        // Alpha blend: original + (fg - original) * weight / 255
        let w = bitmap_weight as u32;
        let inv_w = 255u32 - w;

        let r = (((original.0 as u32 * inv_w + fg_color.0 as u32 * w) * 257) >> 16) as u8;
        let g = (((original.1 as u32 * inv_w + fg_color.1 as u32 * w) * 257) >> 16) as u8;
        let b = (((original.2 as u32 * inv_w + fg_color.2 as u32 * w) * 257) >> 16) as u8;

        // Alpha channel: blend to full opacity based on weight
        let final_a = (((original.3 as u32 * inv_w) * 257) >> 16) + w;
        let final_a = final_a.min(255) as u8;

        (r, g, b, final_a)
    }
}
