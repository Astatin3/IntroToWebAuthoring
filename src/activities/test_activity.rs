use crate::{
    app::{Activity, App, AppState},
    log,
    render::Renderer,
};

pub struct TestActivity {
    elements: Vec<Element>,
}

impl TestActivity {
    pub fn top_element_at_point(&mut self, x: f32, y: f32) -> Option<(&mut Element, usize)> {
        for (i, element) in self.elements.iter_mut().enumerate().rev() {
            if x >= element.x
                && x <= element.x + element.width
                && y >= element.y
                && y <= element.y + element.height
            {
                return Some((element, i));
            }
        }
        None
    }

    pub fn bottom_element_at_point(
        &mut self,
        x: f32,
        y: f32,
        min_index: usize,
    ) -> Option<(&mut Element, usize)> {
        for (i, element) in self.elements.iter_mut().enumerate().skip(min_index) {
            if x >= element.x
                && x <= element.x + element.width
                && y >= element.y
                && y <= element.y + element.height
            {
                return Some((element, i));
            }
        }
        None
    }

    fn mark_for_redraw(&mut self, current_elements: &mut Vec<usize>, element_index: usize) {
        current_elements.push(element_index);

        let element = &self.elements[element_index];

        for (x, y) in vec![
            (element.x.clone(), element.y.clone()),
            (element.x.clone() + element.width, element.y.clone()),
            (element.x.clone(), element.y.clone() + element.height),
            (
                element.x.clone() + element.width,
                element.y.clone() + element.height,
            ),
        ] {
            let option = self.bottom_element_at_point(x, y, element_index + 1);
            if let Some((element, index)) = option {
                if element_index == index || current_elements.contains(&index) {
                    continue;
                }

                self.mark_for_redraw(current_elements, index);
            }
        }
    }
}

impl Activity for TestActivity {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            elements: Vec::new(),
        }
    }

    fn update(&mut self, dt: f32) {
        // todo!()
    }

    fn draw(&self, renderer: &mut Renderer, state: &AppState) {
        renderer.img.randomize(&mut renderer.rand);
        let (cx, cy) = (renderer.actual_width / 2, renderer.actual_height / 2);
        renderer.circle(cx as i32, cy as i32, 100, (127, 127, 127));

        // renderer.rect_center_xywh(cx as i32, cy as i32, 200, 200, (155, 127, 155));
        // renderer.rect_xyxy(10, 10, cx as i32, cy as i32, (255, 127, 155));

        for element in &self.elements {
            element.draw(renderer);
        }

        renderer.update();
    }

    fn l_click(&mut self, renderer: &mut Renderer, state: &AppState) {
        // log!("Mouse moved");
        let option = self.top_element_at_point(state.mouse_x, state.mouse_y);
        if let Some((_, index)) = option {
            let mut for_redraw: Vec<usize> = Vec::new();
            self.mark_for_redraw(&mut for_redraw, index);

            for_redraw.sort();

            for n in for_redraw.iter_mut() {
                let element = &mut self.elements[*n];
                element.color = (
                    renderer.rand.next_u32() as u8,
                    renderer.rand.next_u32() as u8,
                    renderer.rand.next_u32() as u8,
                );
                element.draw(renderer);
            }
            renderer.update();
        } else {
            self.elements.push(Element::new(
                state.mouse_x,
                state.mouse_y,
                50.0,
                50.0,
                (
                    renderer.rand.next_u32() as u8,
                    renderer.rand.next_u32() as u8,
                    renderer.rand.next_u32() as u8,
                ),
            ));

            self.draw(renderer, state);
            renderer.update();
        }
    }

    fn mouse_move(&mut self, renderer: &mut Renderer, state: &AppState) {}
}

pub struct Element {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: (u8, u8, u8),
}

impl Element {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: (u8, u8, u8)) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.rect_xywh(
            self.x as i32,
            self.y as i32,
            self.width as i32,
            self.height as i32,
            self.color,
        );
    }
}
