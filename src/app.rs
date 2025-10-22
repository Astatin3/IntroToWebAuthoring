use wasm_bindgen::prelude::wasm_bindgen;

use crate::render::Renderer;

pub trait Activity {
    fn new() -> Self
    where
        Self: Sized;
    fn update(&mut self, dt: f32);
    fn draw(&self, ctx: &mut Renderer);
}

#[wasm_bindgen]
pub struct App {
    pub(crate) activities: Vec<Box<dyn Activity>>,
    pub(crate) renderer: Renderer,
    pub(crate) current_activity: Option<usize>,
}

impl App {
    pub fn new(renderer: Renderer) -> Self {
        App {
            activities: Vec::new(),
            renderer,
            current_activity: None,
        }
    }

    pub fn draw(&mut self) {
        // if let Some(current_activity) = self.current_activity {
        //     self.activities[current_activity].draw(&mut self.renderer);
        // }
        self.renderer.img.randomize(&mut self.renderer.rand);

        let (cx, cy) = (
            self.renderer.actual_width / 2,
            self.renderer.actual_height / 2,
        );

        self.renderer
            .circle(cx as i32, cy as i32, 200, (255, 255, 255));

        self.renderer.update();
    }
}

// App events
impl App {
    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
        self.draw();
    }

    pub fn click(&mut self, x: f32, y: f32) {
        // self.renderer.click(x, y);
    }
}
