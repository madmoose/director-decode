use crate::riff::chunks::ColorLookupTable;

#[derive(Debug, Default)]
pub struct Palette {
    pub clut: Option<ColorLookupTable>,
}
