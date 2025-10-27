use crate::{
    render::Renderer,
    views::{Bounds, View},
};

pub struct ColorRectView {
    color: (u8, u8, u8),
}

impl ColorRectView {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { color: (r, g, b) }
    }
}

impl View for ColorRectView {
    fn draw(&mut self, renderer: &mut Renderer, x: f32, y: f32, w: f32, h: f32) {
        renderer.rect_xywh(x as i32, y as i32, w as i32, h as i32, self.color);
        // log!("Draw");
    }
    fn bounds(&self, _ph: f32, _pw: f32) -> (Bounds, Bounds) {
        (Bounds::MatchParent, Bounds::Pixels(200.))
    }
}
