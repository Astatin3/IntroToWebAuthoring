use crate::{
    render::Renderer,
    views::{
        Bounds, LayoutView, View,
        constraint_layout::{Constraint, ConstraintView},
    },
};

pub struct ColorRectView {
    color: (u8, u8, u8),
    bounds: (Option<Bounds>, Option<Bounds>),
    constraints: (
        Option<Constraint>,
        Option<Constraint>,
        Option<Constraint>,
        Option<Constraint>,
    ),
}

impl ColorRectView {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            color: (r, g, b),
            bounds: (None, None),
            constraints: (None, None, None, None),
        }
    }
    pub fn set_bounds(&mut self, width: Option<Bounds>, height: Option<Bounds>) {
        self.bounds = (width, height);
    }
    pub fn set_constraints(
        &mut self,
        top: Option<Constraint>,
        left: Option<Constraint>,
        right: Option<Constraint>,
        bottom: Option<Constraint>,
    ) {
        self.constraints = (top, left, right, bottom)
    }
}

impl View for ColorRectView {
    fn draw(&mut self, renderer: &mut Renderer, x: f32, y: f32, w: f32, h: f32) {
        renderer.rect_xywh(x as i32, y as i32, w as i32, h as i32, self.color);
        // log!("Draw");
    }

    fn resize(&mut self, _x: f32, _y: f32, _w: f32, _h: f32) {}

    fn from_tag(attributes: &std::collections::HashMap<String, String>) -> Self
    where
        Self: Sized,
    {
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

        Self::new(r, g, b)
    }
}

impl LayoutView for ColorRectView {
    fn bounds(&self, _ph: f32, _pw: f32) -> (Bounds, Bounds) {
        (
            *self.bounds.0.as_ref().unwrap_or(&Bounds::Pixels(100.)),
            *self.bounds.1.as_ref().unwrap_or(&Bounds::Pixels(100.)),
        )
    }
}

impl ConstraintView for ColorRectView {
    fn get_constraints(
        &self,
    ) -> (
        &Option<Constraint>,
        &Option<Constraint>,
        &Option<Constraint>,
        &Option<Constraint>,
    ) {
        (
            &self.constraints.0,
            &self.constraints.1,
            &self.constraints.2,
            &self.constraints.3,
        )
    }
}
