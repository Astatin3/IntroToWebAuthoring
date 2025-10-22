mod app;
mod render;

use wasm_bindgen::{Clamped, prelude::*};
use web_sys::ImageData;

// use render::Renderer;
use render::buffer::ImgBuffer;
use render::rand::Rnd;

use crate::{app::App, render::Renderer};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn init(canvas: &web_sys::HtmlCanvasElement, width: u32, height: u32) -> App {
    let (cwidth, cheight, dist_x, dist_y) = render::calc_resolution(width, height);
    canvas.set_width(cwidth);
    canvas.set_height(cheight);

    log!("WASM Successfully initialized!");

    let mut renderer = Renderer {
        ctx: canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap(),
        img: ImgBuffer::new(width, height),
        rand: Rnd::new(9825782),

        canvas_width: cwidth,
        canvas_height: cheight,
        actual_width: width,
        actual_height: height,

        distortion_x: dist_x,
        distortion_y: dist_y,
    };

    renderer.resize(width, height);

    App::new(renderer)
}

#[wasm_bindgen]
pub fn resize(app: &mut App, width: u32, height: u32) {
    app.resize(width, height);
}

#[wasm_bindgen]
pub fn draw(app: &mut App) {
    // app.renderer.img.randomize(&mut app.renderer.rand);
    app.draw();
}

#[wasm_bindgen]
pub fn click(app: &mut App, x: i32, y: i32) {
    // ctx.img.randomize(&mut ctx.rand);
    // draw::draw(ctx);

    app.renderer.circle(x, y, 20, (255, 255, 255));

    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&app.renderer.img.data),
        app.renderer.img.width(),
        app.renderer.img.height(),
    )
    .unwrap();
    app.renderer.ctx.put_image_data(&data, 0., 0.).unwrap();
}
