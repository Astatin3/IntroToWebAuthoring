use crate::{
    parser::Tag,
    views::{ColorRectView, ConstraintLayout, View},
};

pub fn interpret_tag(tag: Tag) -> Box<dyn View> {
    match tag.name.as_str() {
        "ColoredRectView" => Box::new(ColorRectView::from_tag(&tag.attributes)),
        "ConstraintLayout" => Box::new(ConstraintLayout::from_tag(&tag.attributes)),

        _ => panic!("Unknown tag: {}", tag.name),
    }
}
