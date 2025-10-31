use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};

use crate::{parser::Tag, render::Renderer};

mod box_view;
mod color_rect_view;
mod constraint_layout;
mod text_view;
mod vertical_layout;

pub use box_view::BoxView;
pub use color_rect_view::ColorRectView;
pub use constraint_layout::ConstraintLayout;

pub trait View: Debug {
    fn draw(&mut self, renderer: &mut Renderer, x: f32, y: f32, w: f32, h: f32);
    fn resize(&mut self, x: f32, y: f32, w: f32, h: f32);

    fn from_tag(attributes: &HashMap<String, String>, children: &Vec<Tag>) -> Box<dyn View>
    where
        Self: Sized;

    fn as_any(self: Box<Self>) -> Box<dyn Any>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bounds {
    MatchParent,
    Pixels(f32),
}

#[derive(Debug)]
pub enum Constraint {
    Left(f32, usize),
    Right(f32, usize),
    Top(f32, usize),
    Bottom(f32, usize),

    LeftParent(f32),
    RightParent(f32),
    TopParent(f32),
    BottomParent(f32),
}

// pub trait ConstraintView: View {
//     fn get_constraints(
//         &self,
//     ) -> (
//         &Option<Constraint>,
//         &Option<Constraint>,
//         &Option<Constraint>,
//         &Option<Constraint>,
//     );
// }

pub fn default_view() -> Box<dyn View> {
    let mut a = BoxView::new(Box::new(ColorRectView::new(127, 0, 0)));

    let temp_margin = 10.0;

    a.set_constraints(
        Some(Constraint::TopParent(temp_margin)),
        None,
        Some(Constraint::Left(temp_margin, 1)),
        None,
        // Some(Constraint::BottomParent(temp_margin)),
    );
    a.set_bounds(Some(Bounds::Pixels(200.)), Some(Bounds::Pixels(100.)));

    let mut b = BoxView::new(Box::new(ColorRectView::new(0, 127, 0)));

    b.set_constraints(
        Some(Constraint::TopParent(temp_margin)),
        None,
        Some(Constraint::RightParent(temp_margin)),
        None,
    );

    let mut c = BoxView::new(Box::new(ColorRectView::new(0, 0, 127)));

    c.set_constraints(
        None,
        Some(Constraint::LeftParent(temp_margin)),
        Some(Constraint::RightParent(temp_margin)),
        Some(Constraint::BottomParent(temp_margin)),
    );

    let mut d = BoxView::new(Box::new(ColorRectView::new(127, 127, 0)));

    d.set_constraints(
        None,
        Some(Constraint::LeftParent(temp_margin)),
        None,
        Some(Constraint::BottomParent(temp_margin)),
    );

    let mut center = BoxView::new(Box::new(ColorRectView::new(127, 127, 0)));

    center.set_constraints(
        Some(Constraint::TopParent(temp_margin)),
        Some(Constraint::LeftParent(temp_margin)),
        Some(Constraint::RightParent(temp_margin)),
        Some(Constraint::BottomParent(temp_margin)),
    );

    center.set_bounds(Some(Bounds::Pixels(350.)), Some(Bounds::Pixels(350.)));

    let mut center_top = BoxView::new(Box::new(ColorRectView::new(127, 0, 127)));

    center_top.set_constraints(
        Some(Constraint::Bottom(0., 4)),
        // None,
        // Some(Constraint::Left(0., 4)),
        None,
        // None,
        Some(Constraint::Left(10., 4)),
        Some(Constraint::Top(0., 4)),
    );

    let mut inside_top = BoxView::new(Box::new(ColorRectView::new(127, 0, 127)));

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
        a, b, c, d, center, center_top, inside_top,
    ]))

    // Box::new(VerticalLayout::new(vec![
    //     Box::new(ColorRectView::new(12, 34, 56)),
    //     Box::new(TextView::new("Testing!\n12345".to_string())),
    //     Box::new(ColorRectView::new(20, 60, 80)),
    // ]))
}
