mod app;
mod fonts;
mod parser;
mod render;
mod views;

use wasm_bindgen::{Clamped, prelude::*};
use web_sys::ImageData;

// use render::Renderer;
use render::buffer::ImgBuffer;
use render::rand::Rnd;

use crate::{
    app::{App, Cursor, set_cursor},
    parser::TEST_XML,
    render::Renderer,
};

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
pub fn init(canvas: &web_sys::HtmlCanvasElement, width: usize, height: usize) -> App {
    log!("WASM Successfully initialized!");

    let mut renderer = Renderer::new(canvas, width, height);
    renderer.resize(width, height);

    App::new(renderer)
}

#[wasm_bindgen]
pub fn resize(app: &mut App, width: usize, height: usize) {
    app.resize(width, height);
}

#[wasm_bindgen]
pub fn draw(app: &mut App) {
    // app.renderer.img.randomize(&mut app.renderer.rand);
    app.draw();
}

#[wasm_bindgen]
pub fn click(app: &mut App, x: f32, y: f32) {
    // ctx.img.randomize(&mut ctx.rand);
    // draw::draw(ctx);
    app.l_click(x, y);
}

#[wasm_bindgen]
pub fn mouse_move(app: &mut App, x: f32, y: f32) {
    app.mouse_move(x, y);
}
