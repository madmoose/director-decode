use crate::riff::chunks::StyledText;

#[derive(Debug, Default)]
pub struct Text {
    pub styled_text: Option<StyledText>,
}
