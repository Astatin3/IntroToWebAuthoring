use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};

use std::fmt::Debug;

use crate::{
    fonts::{FONTS, FontHandle},
    log,
    parser::Tag,
    render::{RESOLUTION, Renderer, buffer::Bitmap},
    views::{Bounds, View},
};

pub struct TextView {
    pub layout: Layout,
    pub text: String,
    pub scale: f32,
    pub font: FontHandle,
}

impl TextView {
    pub fn new(text: String, scale: f32) -> Self {
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);

        layout.reset(&LayoutSettings {
            ..LayoutSettings::default()
        });

        let mut s = Self {
            layout,
            text,
            scale,
            font: FontHandle::AtiksonHyperlegibleRegular,
        };

        let font = (s.font as usize).clone();

        s.layout.clear();

        s.layout
            .append(&FONTS, &TextStyle::new(&s.text, s.scale, font));

        s
    }
}

impl View for TextView {
    fn draw(&mut self, renderer: &mut Renderer, x: f32, y: f32, w: f32, h: f32) {
        let (x, y) = renderer.undistort(x as f32, y as f32);

        let (mut width, mut height): (usize, usize) = (0, 0);
        for glyph in self.layout.glyphs() {
            width = width.max(glyph.x as usize + glyph.width);
            height = height.max(glyph.y as usize + glyph.height);
        }

        let x_padding = 0.;
        let mut new_bitmap = Bitmap::new(width + 2 * (self.scale * x_padding) as usize, height);

        for glyph in self.layout.glyphs() {
            let font = &FONTS[glyph.font_index];
            let (_, char_bitmap) = font.rasterize_config(glyph.key);

            new_bitmap.overlay(
                &Bitmap::from_data(char_bitmap, glyph.width, glyph.height),
                glyph.x as usize,
                glyph.y as usize,
            );
        }

        let scaled = new_bitmap.scale(renderer.ratio_x, renderer.ratio_y);

        renderer
            .img
            .overlay_bitmap_fg(&scaled, x as usize, y as usize, (255, 255, 255));

        // new_bitmap
    }

    fn resize(&mut self, renderer: &mut Renderer, _x: f32, _y: f32, _w: f32, _h: f32) {
        self.layout.clear();

        let font = (self.font as usize).clone();
        let e = (renderer.canvas_height as f32 / renderer.actual_height as f32)
            * (renderer.canvas_width as f32 / renderer.actual_width as f32);

        let scale = self.scale * e;

        self.layout
            .append(&FONTS, &TextStyle::new(&self.text, scale, font));
    }

    fn from_tag(
        attributes: &std::collections::HashMap<String, String>,
        children: &Vec<Tag>,
    ) -> Box<dyn View>
    where
        Self: Sized,
    {
        assert!(children.is_empty(), "TextView does not support children");

        let text = attributes.get("text").unwrap();

        let mut scale = 30.;

        if let Some(font_size_str) = attributes.get("font_size") {
            if let Some(font_size) = font_size_str.parse::<f32>().ok() {
                scale = font_size;
            }
        }

        let this = Self::new(text.clone(), scale);

        Box::new(this)
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

// impl LayoutView for TextView {
//     fn bounds(&self, _ph: f32, _pw: f32) -> (Bounds, Bounds) {
//         // let (x, y) = renderer.undistort(x as f32, y as f32);

//         let (mut width, mut height): (usize, usize) = (0, 0);
//         for glyph in self.layout.glyphs() {
//             width = width.max(glyph.x as usize + glyph.width);
//             height = height.max(glyph.y as usize + glyph.height);
//         }

//         (Bounds::Pixels(width as f32), Bounds::Pixels(height as f32))
//     }
// }

impl Debug for TextView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextView")
            // .field("layout", &self.layout)
            .field("text", &self.text)
            .field("scale", &self.scale)
            .field("font", &self.font)
            .finish()
    }
}
