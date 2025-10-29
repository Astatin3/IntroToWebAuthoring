use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use wasm_bindgen::{Clamped, prelude::*};
use web_sys::ImageData;

use crate::render::{
    RESOLUTION,
    buffer::{Bitmap, ImgBuffer},
    calc_resolution,
    rand::Rnd,
};

#[wasm_bindgen]
pub struct Renderer {
    pub(crate) ctx: web_sys::CanvasRenderingContext2d,
    pub(crate) img: ImgBuffer,
    pub(crate) rand: Rnd,

    pub canvas_width: usize,
    pub canvas_height: usize,
    pub actual_width: usize,
    pub actual_height: usize,

    pub distortion_x: f32,
    pub distortion_y: f32,
    pub ratio_x: f32,
    pub ratio_y: f32,
}

impl Renderer {
    pub fn new(canvas: &web_sys::HtmlCanvasElement, width: usize, height: usize) -> Self {
        let (cwidth, cheight, dist_x, dist_y) = calc_resolution(width, height);

        // let ctx = canvas.get_context("2d").unwrap().unwrap();
        let img = ImgBuffer::new(cwidth, cheight);
        let rand = Rnd::new(12345);

        Self {
            ctx: canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap(),
            img,
            rand,

            canvas_width: cwidth,
            canvas_height: cheight,
            actual_width: width,
            actual_height: height,
            distortion_x: dist_x,
            distortion_y: dist_y,
            ratio_x: cwidth as f32 / width as f32,
            ratio_y: cheight as f32 / height as f32,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        let (cwidth, cheight, dist_x, dist_y) = calc_resolution(width, height);

        self.canvas_width = cwidth;
        self.canvas_height = cheight;
        self.actual_width = width;
        self.actual_height = height;
        self.distortion_x = dist_x;
        self.distortion_y = dist_y;

        self.ratio_x = cwidth as f32 / width as f32;
        self.ratio_y = cheight as f32 / height as f32;

        self.img.resize(cwidth, cheight);
        self.ctx.canvas().unwrap().set_width(cwidth as u32);
        self.ctx.canvas().unwrap().set_height(cheight as u32);
    }

    pub fn update(&mut self) {
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.img.data),
            self.img.width() as u32,
            self.img.height() as u32,
        )
        .unwrap();
        self.ctx.put_image_data(&data, 0., 0.).unwrap();
    }

    pub fn clear(&mut self) {
        self.img.data = vec![0; (self.img.width() * self.img.height() * 4) as usize];
    }
}

impl Renderer {
    pub fn undistort(&self, x: f32, y: f32) -> (f32, f32) {
        (x * self.ratio_x, y * self.ratio_y)
    }

    /// Draw a rectangle centered at (cx, cy) with the given width and height.
    pub fn rect_center_xywh(
        &mut self,
        cx: i32,
        cy: i32,
        width: i32,
        height: i32,
        color: (u8, u8, u8),
    ) {
        self.rect_xyxy(
            cx - width / 2,
            cy - height / 2,
            cx + width / 2,
            cy + height / 2,
            color,
        );
    }

    /// Draw a rectangle at (x, y) with the given width and height.
    pub fn rect_xywh(&mut self, x: i32, y: i32, width: i32, height: i32, color: (u8, u8, u8)) {
        self.rect_xyxy(x, y, x + width, y + height, color);
    }

    /// Draw a rectangle at (x1, y1) with the given width and height.
    pub fn rect_xyxy(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: (u8, u8, u8)) {
        let (leftx, topy) = self.undistort(x1 as f32, y1 as f32);
        let (rightx, bottomy) = self.undistort(x2 as f32, y2 as f32);

        let leftx = leftx.max(0f32);
        let topy = topy.max(0f32);
        let rightx = rightx.min(self.canvas_width as f32);
        let bottomy = bottomy.min(self.canvas_height as f32);

        for x in leftx as i32..rightx as i32 {
            for y in topy as i32..bottomy as i32 {
                self.img.set_pixel(x as usize, y as usize, color);
            }
        }
    }

    /// Calculate the square distance between two points on the screen, and convert to canvas coordinates for processing
    pub fn screen_dist_sq(&self, x: f32, y: f32, rx: f32, ry: f32) -> f32 {
        (rx - x).powf(2.) * self.distortion_y + (ry - y).powf(2.) * self.distortion_x
    }

    pub fn circle(&mut self, cx: i32, cy: i32, radius: i32, color: (u8, u8, u8)) {
        let radius = radius as f32;

        let (leftx, topy) = self.undistort(cx as f32 - radius, cy as f32 - radius);
        let (rightx, bottomy) = self.undistort(cx as f32 + radius, cy as f32 + radius);

        let (cx, cy) = self.undistort(cx as f32, cy as f32);

        let leftx = leftx.max(0f32);
        let topy = topy.max(0f32);
        let rightx = rightx.min(self.canvas_width as f32);
        let bottomy = bottomy.min(self.canvas_height as f32);

        // let e = (self.canvas_height as f32 / self.actual_height as f32)
        //     * (self.canvas_width as f32 / self.actual_width as f32);

        let e = RESOLUTION.pow(2) as f32 / (self.actual_width * self.actual_height) as f32;

        let r2 = (radius).powf(2.) * e;

        // log!("{}, {}, {}, {}", leftx, rightx, topy, bottomy);

        for x in leftx as i32..rightx as i32 {
            for y in topy as i32..bottomy as i32 {
                if self.screen_dist_sq(x as f32, y as f32, cx, cy) <= r2 {
                    self.img.set_pixel(x as usize, y as usize, color);
                }
            }
        }
    }
}

// // Fonts
// impl Renderer {
//     pub fn rasterize_font(
//         &mut self,
//         x: i32,
//         y: i32,
//         text: &str,
//         font_scale: f32,
//         font: FontHandle,
//     ) {
//     }

//     pub fn measure_text_bounds(
//         &mut self,
//         text: &str,
//         font_scale: i32,
//         font: FontHandle,
//     ) -> (i32, i32) {
//         (100, 100)
//     }
// }
