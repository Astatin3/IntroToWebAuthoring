use crate::views::{Bounds, View};

pub struct VerticalLayout {
    pub views: Vec<Box<dyn View>>,
}

impl VerticalLayout {
    pub fn new(views: Vec<Box<dyn View>>) -> Self {
        Self { views }
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
            };

            view.draw(renderer, x, cur_y, w.min(vx), vy);
            cur_y += vy;
            if cur_y > h {
                return;
            }
        }
    }

    fn bounds(&self, pw: f32, ph: f32) -> (Bounds, Bounds) {
        let (mut maxx, mut totaly): (f32, f32) = (0., 0.);
        for view in &self.views {
            let (vx, vy) = view.bounds(pw, ph);

            let vx = match vx {
                super::Bounds::MatchParent => pw,
                super::Bounds::Pixels(x) => x,
            };

            let vy = match vy {
                super::Bounds::MatchParent => ph,
                super::Bounds::Pixels(x) => x,
            };

            maxx = maxx.max(vx);
            totaly += vy;
        }

        (Bounds::Pixels(maxx), Bounds::Pixels(totaly))
    }
}
