use wasm_bindgen::{Clamped, prelude::*};
use web_sys::ImageData;

use crate::{
    log,
    render::{buffer::ImgBuffer, calc_resolution, rand::Rnd},
};

#[wasm_bindgen]
pub struct Renderer {
    pub(crate) ctx: web_sys::CanvasRenderingContext2d,
    pub(crate) img: ImgBuffer,
    pub(crate) rand: Rnd,

    pub canvas_width: u32,
    pub canvas_height: u32,
    pub actual_width: u32,
    pub actual_height: u32,

    pub distortion_x: f32,
    pub distortion_y: f32,
}

impl Renderer {
    pub fn new(canvas: &web_sys::HtmlCanvasElement, width: u32, height: u32) -> Self {
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
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let (cwidth, cheight, dist_x, dist_y) = calc_resolution(width, height);

        self.canvas_width = cwidth;
        self.canvas_height = cheight;
        self.actual_width = width;
        self.actual_height = height;
        self.distortion_x = dist_x;
        self.distortion_y = dist_y;

        self.img.resize(cwidth, cheight);
        self.ctx.canvas().unwrap().set_width(cwidth);
        self.ctx.canvas().unwrap().set_height(cheight);
    }

    pub fn update(&mut self) {
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.img.data),
            self.img.width(),
            self.img.height(),
        )
        .unwrap();
        self.ctx.put_image_data(&data, 0., 0.).unwrap();
    }
}

impl Renderer {
    pub fn undistort(&self, x: f32, y: f32) -> (f32, f32) {
        (
            x * (self.canvas_width as f32 / self.actual_width as f32),
            y * (self.canvas_height as f32 / self.actual_height as f32),
        )
    }

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

        let e = (self.canvas_height as f32 / self.actual_height as f32)
            * (self.canvas_width as f32 / self.actual_width as f32);

        let r2 = (radius).powf(2.) * e;

        // log!("{}, {}, {}, {}", leftx, rightx, topy, bottomy);

        for x in leftx as i32..rightx as i32 {
            for y in topy as i32..bottomy as i32 {
                if self.screen_dist_sq(x as f32, y as f32, cx, cy) <= r2 {
                    self.img.set_pixel(x as u32, y as u32, color);
                }
            }
        }
    }
}
