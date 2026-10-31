use std::collections::HashMap;

use crate::{
    parser::Tag,
    render::Renderer,
    views::{Bounds, View, box_view::BoxView},
};

#[derive(Debug)]
pub struct VerticalLayout {
    pub views: Vec<BoxView>,
    pub bounds: (Option<Bounds>, Option<Bounds>),
}

impl VerticalLayout {
    pub fn new(views: Vec<BoxView>) -> Self {
        Self {
            views,
            bounds: (None, None),
        }
    }
    pub fn set_bounds(&mut self, width: Option<Bounds>, height: Option<Bounds>) {
        self.bounds = (width, height);
    }
}

impl View for VerticalLayout {
    fn draw(&mut self, renderer: &mut crate::render::Renderer, x: f32, y: f32, w: f32, h: f32) {
        let mut cur_y = y;

        for view in &mut self.views {
            let (vx, vy) = view.bounds(w, h);

            let vx = match vx {
                super::Bounds::MatchParent => w,
                super::Bounds::Pixels(x) => x,
            };

            let vy = match vy {
                super::Bounds::MatchParent => h,
                super::Bounds::Pixels(x) => x,
            } + view.get_top_margin();

            view.draw(renderer, x, cur_y, w.min(vx), vy);

            cur_y += vy + view.get_bottom_margin();

            if cur_y > h {
                return;
            }
        }
    }
    fn resize(&mut self, renderer: &mut Renderer, x: f32, y: f32, w: f32, h: f32) {}

    fn from_tag(attributes: &HashMap<String, String>, children: &Vec<Tag>) -> Box<dyn View>
    where
        Self: Sized,
    {
        let mut parsed_children = Vec::new();

        for child in children {
            let child = child.parse();

            if let Ok(b) = child.as_any().downcast::<BoxView>() {
                parsed_children.push(*b);
            } else {
                panic!("Constraint Layout must contain only BoxView!")
            }
        }

        Box::new(Self::new(parsed_children))

        // Box::new(Self {
        //     views: children.iter().map(|tag| tag.parse()).collect(),
        //     bounds: (None, None),
        // })
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

// impl LayoutView for VerticalLayout {
//     fn bounds(&self, pw: f32, ph: f32) -> (Bounds, Bounds) {
//         let (mut maxx, mut totaly): (f32, f32) = (0., 0.);
//         for view in &self.views {
//             let (vx, vy) = view.bounds(pw, ph);

//             let vx = match vx {
//                 super::Bounds::MatchParent => pw,
//                 super::Bounds::Pixels(x) => x,
//             };

//             let vy = match vy {
//                 super::Bounds::MatchParent => ph,
//                 super::Bounds::Pixels(x) => x,
//             };

//             maxx = maxx.max(vx);
//             totaly += vy;
//         }

//         (Bounds::Pixels(maxx), Bounds::Pixels(totaly))
//     }
// }
