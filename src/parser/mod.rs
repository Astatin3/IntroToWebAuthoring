use std::collections::HashMap;

use crate::views::View;

mod interpreter;
mod parser;

pub const TEST_XML: &'static str = include_str!("../../pages/main.xml");

#[derive(Debug)]
struct Tag {
    name: String,
    children: Vec<Tag>,
    attributes: HashMap<String, String>,
}

pub fn parse(xml: &str) -> Box<dyn View> {
    let tag = parser::parse_xml(xml);
    interpreter::interpret_tag(tag)
}
