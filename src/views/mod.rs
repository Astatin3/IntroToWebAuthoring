use std::collections::HashMap;

use crate::{
    render::Renderer,
    views::constraint_layout::{Constraint, ConstraintView},
};

mod color_rect_view;
mod constraint_layout;
mod text_view;
mod vertical_layout;

pub use color_rect_view::ColorRectView;
pub use constraint_layout::ConstraintLayout;
pub use text_view::TextView;
pub use vertical_layout::VerticalLayout;

pub trait View {
    fn draw(&mut self, renderer: &mut Renderer, x: f32, y: f32, w: f32, h: f32);
    fn resize(&mut self, x: f32, y: f32, w: f32, h: f32);

    fn from_tag(attributes: &HashMap<String, String>) -> Self
    where
        Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bounds {
    MatchParent,
    Pixels(f32),
}

pub trait LayoutView: View {
    // fn set_bounds(&mut self, width: Option<Bounds>, height: Option<Bounds>);
    fn bounds(&self, pw: f32, ph: f32) -> (Bounds, Bounds);
}

// pub trait ViewPosition: Drawable {
//     // fn draw(&self)
// }

pub fn default_view() -> Box<dyn View> {
    let mut a = ColorRectView::new(127, 0, 0);

    let temp_margin = 10.0;

    a.set_constraints(
        Some(Constraint::TopParent(temp_margin)),
        None,
        Some(Constraint::Left(temp_margin, 1)),
        None,
        // Some(Constraint::BottomParent(temp_margin)),
    );
    a.set_bounds(Some(Bounds::Pixels(200.)), Some(Bounds::Pixels(100.)));

    let mut b = ColorRectView::new(0, 127, 0);

    b.set_constraints(
        Some(Constraint::TopParent(temp_margin)),
        None,
        Some(Constraint::RightParent(temp_margin)),
        None,
    );

    let mut c = ColorRectView::new(0, 0, 127);

    c.set_constraints(
        None,
        Some(Constraint::LeftParent(temp_margin)),
        Some(Constraint::RightParent(temp_margin)),
        Some(Constraint::BottomParent(temp_margin)),
    );

    let mut d = ColorRectView::new(127, 127, 0);

    d.set_constraints(
        None,
        Some(Constraint::LeftParent(temp_margin)),
        None,
        Some(Constraint::BottomParent(temp_margin)),
    );

    let mut center = ColorRectView::new(127, 127, 0);

    center.set_constraints(
        Some(Constraint::TopParent(temp_margin)),
        Some(Constraint::LeftParent(temp_margin)),
        Some(Constraint::RightParent(temp_margin)),
        Some(Constraint::BottomParent(temp_margin)),
    );

    center.set_bounds(Some(Bounds::Pixels(350.)), Some(Bounds::Pixels(350.)));

    let mut center_top = ColorRectView::new(127, 0, 127);

    center_top.set_constraints(
        Some(Constraint::Bottom(0., 4)),
        // None,
        // Some(Constraint::Left(0., 4)),
        None,
        // None,
        Some(Constraint::Left(10., 4)),
        Some(Constraint::Top(0., 4)),
    );

    let mut inside_top = ColorRectView::new(127, 0, 127);

    inside_top.set_constraints(
        Some(Constraint::Bottom(0., 4)),
        // None,
        // Some(Constraint::Left(0., 4)),
        None,
        // None,
        Some(Constraint::Right(10., 4)),
        Some(Constraint::Top(0., 4)),
    );

    Box::new(ConstraintLayout::new(vec![
        Box::new(a),
        Box::new(b),
        Box::new(c),
        Box::new(d),
        Box::new(center),
        Box::new(center_top),
        Box::new(inside_top),
    ]))

    // Box::new(VerticalLayout::new(vec![
    //     Box::new(ColorRectView::new(12, 34, 56)),
    //     Box::new(TextView::new("Testing!\n12345".to_string())),
    //     Box::new(ColorRectView::new(20, 60, 80)),
    // ]))
}
