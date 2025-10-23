mod cursors;

pub use cursors::{Cursor, set_cursor};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    log,
    render::Renderer,
    views::{View, default_view},
};

pub trait Activity {
    fn new() -> Self
    where
        Self: Sized;
    fn update(&mut self, dt: f32);
    fn draw(&self, renderer: &mut Renderer, state: &AppState);
    fn l_click(&mut self, renderer: &mut Renderer, state: &AppState);
    fn mouse_move(&mut self, renderer: &mut Renderer, state: &AppState);
}

#[wasm_bindgen]
pub struct App {
    // pub(crate) activities: Vec<Box<dyn Activity>>,
    pub(crate) root_view: Option<Box<dyn View>>,
    pub(crate) renderer: Renderer,
    // pub(crate) current_activity: Option<usize>,
    pub(crate) state: AppState,
}

pub struct AppState {
    pub(crate) mouse_x: f32,
    pub(crate) mouse_y: f32,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }
}

impl App {
    pub fn new(renderer: Renderer) -> Self {
        App {
            root_view: Some(default_view()),
            renderer,
            // current_activity: Some(0),
            state: AppState::new(),
        }
    }

    pub fn draw(&mut self) {
        if let Some(view) = &self.root_view {
            let (width, height) = (
                self.renderer.actual_width.clone() as f32,
                self.renderer.actual_height.clone() as f32,
            );

            view.draw(&mut self.renderer, 0., 0., width, height);
            self.renderer.update();
        }
    }
}

// App events
impl App {
    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
        self.draw();
    }

    pub fn mouse_move(&mut self, x: f32, y: f32) {
        self.state.mouse_x = x;
        self.state.mouse_y = y;

        // if let Some(current_activity) = self.current_activity {
        //     self.activities[current_activity].mouse_move(&mut self.renderer, &self.state);
        // }
    }

    pub fn l_click(&mut self, x: f32, y: f32) {
        self.state.mouse_x = x;
        self.state.mouse_y = y;

        // if let Some(current_activity) = self.current_activity {
        //     self.activities[current_activity].l_click(&mut self.renderer, &self.state);
        // }
    }
}
