use std::any::TypeId;

use crate::{
    log,
    parser::Tag,
    render::Renderer,
    views::{Bounds, Constraint, View},
};

#[derive(Debug)]
pub struct BoxView {
    // view: Box<dyn View>,
    views: Vec<Box<dyn View>>,
    bounds: (Option<Bounds>, Option<Bounds>),
    constraints: (
        Option<Constraint>,
        Option<Constraint>,
        Option<Constraint>,
        Option<Constraint>,
    ),
}

impl BoxView {
    pub fn new(views: Vec<Box<dyn View>>) -> Self {
        Self {
            views,
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
    pub fn bounds(&self, _ph: f32, _pw: f32) -> (Bounds, Bounds) {
        (
            *self.bounds.0.as_ref().unwrap_or(&Bounds::Pixels(100.)),
            *self.bounds.1.as_ref().unwrap_or(&Bounds::Pixels(100.)),
        )
    }
    pub fn get_constraints(
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

impl View for BoxView {
    fn draw(&mut self, renderer: &mut Renderer, x: f32, y: f32, w: f32, h: f32) {
        for view in &mut self.views {
            view.draw(renderer, x, y, w, h);
        }
    }

    fn resize(&mut self, renderer: &mut Renderer, x: f32, y: f32, w: f32, h: f32) {
        for view in &mut self.views {
            view.resize(renderer, x, y, w, h);
        }
    }

    fn from_tag(
        attributes: &std::collections::HashMap<String, String>,
        children: &Vec<Tag>,
    ) -> Box<dyn View> {
        // assert!(
        //     children.len() >= 1,
        //     "BoxView must have greater than one child"
        // );

        let (mut margin_top, mut margin_left, mut margin_right, mut margin_bottom) =
            (None, None, None, None);

        let (mut align_top, mut align_left, mut align_right, mut align_bottom) =
            (None, None, None, None);

        let parse_float = |var: &mut Option<f32>, value: &str| {
            if let Ok(float) = value.parse::<f32>() {
                *var = Some(float);
            } else if let Ok(int) = value.parse::<i32>() {
                let float = int as f32;
                *var = Some(float);
            } else {
                panic!("Could not parse margin!")
            }
        };

        let parse_align_refrence = |value: &String| -> usize {
            match value.as_str() {
                "parent" => 0,
                _ => {
                    if let Ok(parent) = value.parse::<usize>() {
                        if parent >= children.len() {
                            // panic!("Child {} does not exist!", parent);
                        }
                        parent
                    } else {
                        panic!("Could not parse refrence");
                    }
                }
            }
        };

        for (key, value) in attributes {
            match key.as_str() {
                // Set margin on all sides
                "margin" => {
                    if let Ok(float) = value.parse::<f32>() {
                        (margin_top, margin_left, margin_right, margin_bottom) =
                            (Some(float), Some(float), Some(float), Some(float));
                    } else if let Ok(int) = value.parse::<i32>() {
                        let float = int as f32;
                        (margin_top, margin_left, margin_right, margin_bottom) =
                            (Some(float), Some(float), Some(float), Some(float));
                    } else {
                        panic!("Could not parse margin!")
                    }
                }

                // Margin for specific directions
                "margin_top" => parse_float(&mut margin_top, value),
                "margin_left" => parse_float(&mut margin_left, value),
                "margin_right" => parse_float(&mut margin_right, value),
                "margin_bottom" => parse_float(&mut margin_bottom, value),
                _ => {}
            }
        }

        for (key, value) in attributes {
            match key.as_str() {
                // Alignment
                "align_top_to_top" => {
                    align_top = Some(match parse_align_refrence(value) {
                        0 => Constraint::TopParent(margin_top.unwrap_or(0.)),
                        n => Constraint::Top(margin_top.unwrap_or(0.), n - 1),
                    })
                }
                "align_top_to_bottom" => {
                    align_top = Some(match parse_align_refrence(value) {
                        0 => Constraint::BottomParent(margin_top.unwrap_or(0.)),
                        n => Constraint::Bottom(margin_top.unwrap_or(0.), n - 1),
                    })
                }

                "align_left_to_left" => {
                    align_left = Some(match parse_align_refrence(value) {
                        0 => Constraint::LeftParent(margin_left.unwrap_or(0.)),
                        n => Constraint::Left(margin_left.unwrap_or(0.), n - 1),
                    })
                }
                "align_left_to_right" => {
                    align_left = Some(match parse_align_refrence(value) {
                        0 => Constraint::RightParent(margin_left.unwrap_or(0.)),
                        n => Constraint::Right(margin_left.unwrap_or(0.), n - 1),
                    })
                }

                "align_right_to_right" => {
                    align_right = Some(match parse_align_refrence(value) {
                        0 => Constraint::RightParent(margin_right.unwrap_or(0.)),
                        n => Constraint::Right(margin_right.unwrap_or(0.), n - 1),
                    })
                }
                "align_right_to_left" => {
                    align_right = Some(match parse_align_refrence(value) {
                        0 => Constraint::LeftParent(margin_right.unwrap_or(0.)),
                        n => Constraint::Left(margin_right.unwrap_or(0.), n - 1),
                    })
                }

                "align_bottom_to_bottom" => {
                    align_bottom = Some(match parse_align_refrence(value) {
                        0 => Constraint::BottomParent(margin_bottom.unwrap_or(0.)),
                        n => Constraint::Bottom(margin_bottom.unwrap_or(0.), n - 1),
                    })
                }
                "align_bottom_to_top" => {
                    align_bottom = Some(match parse_align_refrence(value) {
                        0 => Constraint::TopParent(margin_bottom.unwrap_or(0.)),
                        n => Constraint::Top(margin_bottom.unwrap_or(0.), n - 1),
                    })
                }
                _ => {}
            }
        }
        let views = children.iter().map(|c| c.parse()).collect();

        let mut this = Self::new(views);

        this.constraints = (align_top, align_left, align_right, align_bottom);
        this.bounds = Tag::parse_bounds(attributes);

        Box::new(this)
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
