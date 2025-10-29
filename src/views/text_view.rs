use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};

use crate::{
    fonts::{FONTS, FontHandle},
    render::{Renderer, buffer::Bitmap},
    views::{Bounds, LayoutView, View},
};

pub struct TextView {
    layout: Layout,
    text: String,
    scale: f32,
    font: FontHandle,
    bounds: (Option<Bounds>, Option<Bounds>),
}

impl TextView {
    pub fn new(text: String) -> Self {
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);

        layout.reset(&LayoutSettings {
            ..LayoutSettings::default()
        });

        let mut s = Self {
            layout,
            text,
            scale: 20.,
            font: FontHandle::AtiksonHyperlegibleRegular,
            bounds: (None, None),
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
        // renderer.rasterize_font(
        //     x as i32,
        //     y as i32,
        //     &self.text,
        //     20.,
        //     FontHandle::AtiksonHyperlegibleRegular,
        // );
        // renderer.

        // renderer.rect_xywh(x as i32, y as i32, w as i32, h as i32, self.color);

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

        renderer.img.overlay_bitmap(&scaled, x as usize, y as usize);

        // new_bitmap
    }
}

impl LayoutView for TextView {
    fn set_bounds(&mut self, width: Option<Bounds>, height: Option<Bounds>) {
        self.bounds = (width, height);
    }
    fn bounds(&self, _ph: f32, _pw: f32) -> (Bounds, Bounds) {
        // let (x, y) = renderer.undistort(x as f32, y as f32);

        let (mut width, mut height): (usize, usize) = (0, 0);
        for glyph in self.layout.glyphs() {
            width = width.max(glyph.x as usize + glyph.width);
            height = height.max(glyph.y as usize + glyph.height);
        }

        (Bounds::Pixels(width as f32), Bounds::Pixels(height as f32))
    }
}
