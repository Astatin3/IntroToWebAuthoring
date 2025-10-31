use crate::{
    parser::Tag,
    render::Renderer,
    views::{Bounds, Constraint, View},
};

#[derive(Debug)]
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

    fn resize(&mut self, _x: f32, _y: f32, _w: f32, _h: f32) {}

    fn from_tag(
        attributes: &std::collections::HashMap<String, String>,
        children: &Vec<Tag>,
    ) -> Box<dyn View>
    where
        Self: Sized,
    {
        assert!(
            children.is_empty(),
            "ColorRectView does not support children"
        );

        let r = attributes
            .get("r")
            .unwrap_or(&"0".to_string())
            .parse()
            .unwrap();
        let g = attributes
            .get("g")
            .unwrap_or(&"0".to_string())
            .parse()
            .unwrap();
        let b = attributes
            .get("b")
            .unwrap_or(&"0".to_string())
            .parse()
            .unwrap();

        Box::new(Self::new(r, g, b))
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
