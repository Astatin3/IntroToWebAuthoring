use crate::render::Renderer;

mod color_rect_view;
mod text_view;
mod vertical_layout;

use color_rect_view::ColorRectView;
use text_view::TextView;
use vertical_layout::VerticalLayout;

pub trait View {
    fn draw(&mut self, renderer: &mut Renderer, x: f32, y: f32, w: f32, h: f32);
    fn bounds(&self, pw: f32, ph: f32) -> (Bounds, Bounds);
}

pub enum Bounds {
    MatchParent,
    Pixels(f32),
}

// pub trait ViewPosition: Drawable {
//     // fn draw(&self)
// }

pub fn default_view() -> Box<dyn View> {
    Box::new(VerticalLayout::new(vec![
        Box::new(ColorRectView::new(12, 34, 56)),
        Box::new(TextView::new("Testing!\n12345".to_string())),
        Box::new(ColorRectView::new(20, 60, 80)),
    ]))
}
