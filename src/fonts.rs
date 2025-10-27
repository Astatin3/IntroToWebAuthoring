use fontdue::Font;
use lazy_static::lazy_static;

// Include the fonts in the binary
pub static ATKINSON_HYPERLEGIBLE_REGULAR: &'static [u8] =
    include_bytes!("../fonts/Atkinson_Hyperlegible/AtkinsonHyperlegible-Regular.ttf");

// Parse the fonts at runtime
lazy_static! {
    pub static ref FONTS: Vec<Font> = vec![
        Font::from_bytes(
            ATKINSON_HYPERLEGIBLE_REGULAR,
            fontdue::FontSettings::default()
        )
        .unwrap(),
    ];
}

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FontHandle {
    AtiksonHyperlegibleRegular = 0,
}
