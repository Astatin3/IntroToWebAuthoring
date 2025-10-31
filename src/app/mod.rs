mod cursors;

pub use cursors::{Cursor, set_cursor};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    log,
    parser::{self, TEST_XML},
    render::Renderer,
    views::View,
};

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
        let (width, height) = (renderer.actual_width, renderer.actual_height);

        let root_view = parser::parse(TEST_XML);
        log!("root: {:?}", root_view);

        let mut this = App {
            root_view: Some(root_view),
            renderer,
            // current_activity: Some(0),
            state: AppState::new(),
        };

        this.resize(width, height);

        this
    }

    pub fn draw(&mut self) {
        // self.renderer.img.randomize(&mut self.renderer.rand);
        if let Some(view) = &mut self.root_view {
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
    pub fn resize(&mut self, width: usize, height: usize) {
        self.renderer.resize(width, height);
        if let Some(view) = &mut self.root_view {
            view.resize(0., 0., width as f32, height as f32);
        }
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
