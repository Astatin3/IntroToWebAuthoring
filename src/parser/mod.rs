use std::collections::HashMap;

use crate::{
    log,
    views::{Bounds, BoxView, ColorRectView, ConstraintLayout, TextView, View},
};

mod parser;

pub const TEST_XML: &'static str = include_str!("../../pages/main.xml");

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub children: Vec<Tag>,
    pub attributes: HashMap<String, String>,
}

impl Tag {
    pub fn parse(&self) -> Box<dyn View> {
        match self.name.as_str() {
            "ColoredRectView" => ColorRectView::from_tag(&self.attributes, &self.children),
            "ConstraintLayout" => ConstraintLayout::from_tag(&self.attributes, &self.children),
            "BoxView" => BoxView::from_tag(&self.attributes, &self.children),
            "TextView" => TextView::from_tag(&self.attributes, &self.children),

            _ => panic!("Unknown tag: {}", self.name),
        }
    }
    pub fn parse_bounds(attributes: &HashMap<String, String>) -> (Option<Bounds>, Option<Bounds>) {
        let (mut width, mut height) = (None, None);

        let parse_bound = |str: &str| -> Bounds {
            match str {
                "parent" => Bounds::MatchParent,
                _ => {
                    let bound = if let Ok(int) = str.parse::<i32>() {
                        int as f32
                    } else if let Ok(float) = str.parse::<f32>() {
                        float
                    } else {
                        panic!("Invalid String");
                    };

                    Bounds::Pixels(bound)
                }
            }
        };

        for (key, value) in attributes {
            match key.as_str() {
                "width" => width = Some(parse_bound(value)),
                "height" => height = Some(parse_bound(value)),
                _ => {}
            }
        }

        (width, height)
    }
}

pub fn parse(xml: &str) -> Box<dyn View> {
    parser::parse_xml(xml).parse()
}
