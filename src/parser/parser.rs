use std::collections::HashMap;

use quick_xml::{
    Reader,
    events::{BytesStart, Event},
};

use crate::{log, parser::Tag};

pub fn parse_xml(xml: &str) -> Tag {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    // let mut count = 0;
    // let mut txt = Vec::new();
    let mut buf = Vec::new();

    let mut tag = None;

    loop {
        let event = reader.read_event_into(&mut buf);

        match event {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                tag = Some(parse_tag(e, &mut reader));
            }
            Ok(Event::Empty(e)) => {
                tag = Some(parse_tag_empty(e));
            }
            Ok(Event::End(_)) => {
                break;
            }
            _ => {}
        };
        buf.clear();
    }

    if tag.is_none() {
        log!("No root tag found");
    }

    tag.unwrap()
}

fn parse_tag(event: BytesStart<'_>, reader: &mut Reader<&[u8]>) -> Tag {
    let mut children = Vec::new();

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => {
                log!("Error at position {}: {:?}", reader.error_position(), e);
                panic!()
            }
            // exits the loop when reaching end of file
            Ok(Event::Eof) => {
                log!("Unexpected end of file");
                panic!()
            }

            Ok(Event::Start(e)) => {
                children.push(parse_tag(e, reader));
            }
            Ok(Event::Empty(e)) => {
                children.push(parse_tag_empty(e));
            }
            Ok(Event::End(e)) => {
                println!("End: {:?}", e);
                break;
            }
            _ => (),
        }
        buf.clear();
    }

    let name = String::from_utf8_lossy(event.name().as_ref()).to_string();

    let attributes = event
        .attributes()
        .map(|a| {
            let a = a.unwrap();
            (
                String::from_utf8_lossy(a.key.as_ref()).to_string(),
                String::from_utf8_lossy(a.value.as_ref()).to_string(),
            )
        })
        .collect::<HashMap<_, _>>();

    Tag {
        name,
        children,
        attributes,
    }
}

fn parse_tag_empty(event: BytesStart<'_>) -> Tag {
    let name = String::from_utf8_lossy(event.name().as_ref()).to_string();

    let attributes = event
        .attributes()
        .map(|a| {
            let a = a.unwrap();
            (
                String::from_utf8_lossy(a.key.as_ref()).to_string(),
                String::from_utf8_lossy(a.value.as_ref()).to_string(),
            )
        })
        .collect::<HashMap<_, _>>();

    Tag {
        name,
        children: Vec::new(),
        attributes,
    }
}
