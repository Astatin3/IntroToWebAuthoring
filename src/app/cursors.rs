use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/js/index.js")]
extern "C" {
    fn cursor(name: &str);
}

pub fn set_cursor(c: Cursor) {
    cursor(c.value());
}

#[allow(dead_code)]
pub enum Cursor {
    Alias,
    AllScroll,
    Auto,
    Cell,
    ColResize,
    ContextMenu,
    Copy,
    Crosshair,
    Default,
    EResize,
    EWResize,
    Grab,
    Grabbing,
    Help,
    Move,
    NResize,
    NEResize,
    NESWResize,
    NSResize,
    NWResize,
    NWSEResize,
    NoDrop,
    None,
    NotAllowed,
    Pointer,
    Progress,
    RowResize,
    SResize,
    SEResize,
    SWResize,
    Text,
    WResize,
    Wait,
    ZoomIn,
    ZoomOut,
}

impl Cursor {
    fn value(&self) -> &str {
        match self {
            Cursor::Alias => "alias",
            Cursor::AllScroll => "all-scroll",
            Cursor::Auto => "auto",
            Cursor::Cell => "cell",
            Cursor::ColResize => "col-resize",
            Cursor::ContextMenu => "context-menu",
            Cursor::Copy => "copy",
            Cursor::Crosshair => "crosshair",
            Cursor::Default => "default",
            Cursor::EResize => "e-resize",
            Cursor::EWResize => "ew-resize",
            Cursor::Grab => "grab",
            Cursor::Grabbing => "grabbing",
            Cursor::Help => "help",
            Cursor::Move => "move",
            Cursor::NResize => "n-resize",
            Cursor::NEResize => "ne-resize",
            Cursor::NESWResize => "nesw-resize",
            Cursor::NSResize => "ns-resize",
            Cursor::NWResize => "nw-resize",
            Cursor::NWSEResize => "nwse-resize",
            Cursor::NoDrop => "no-drop",
            Cursor::None => "none",
            Cursor::NotAllowed => "not-allowed",
            Cursor::Pointer => "pointer",
            Cursor::Progress => "progress",
            Cursor::RowResize => "row-resize",
            Cursor::SResize => "s-resize",
            Cursor::SEResize => "se-resize",
            Cursor::SWResize => "sw-resize",
            Cursor::Text => "text",
            Cursor::WResize => "w-resize",
            Cursor::Wait => "wait",
            Cursor::ZoomIn => "zoom-in",
            Cursor::ZoomOut => "zoom-out",
        }
    }
}
