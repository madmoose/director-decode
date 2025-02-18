use std::fmt;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum CastMemberType {
    Null,
    Bitmap,
    FilmLoop,
    Text,
    Palette,
    Picture,
    Sound,
    Button,
    Shape,
    Movie,
    DigitalVideo,
    Script,
    RTE,
}

#[derive(Debug, PartialEq)]
pub struct InvalidCastMemberTypeError(u8);

impl fmt::Display for InvalidCastMemberTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid CastMemberType value: {}", self.0)
    }
}

impl std::error::Error for InvalidCastMemberTypeError {}

impl TryFrom<u8> for CastMemberType {
    type Error = InvalidCastMemberTypeError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(CastMemberType::Null),
            1 => Ok(CastMemberType::Bitmap),
            2 => Ok(CastMemberType::FilmLoop),
            3 => Ok(CastMemberType::Text),
            4 => Ok(CastMemberType::Palette),
            5 => Ok(CastMemberType::Picture),
            6 => Ok(CastMemberType::Sound),
            7 => Ok(CastMemberType::Button),
            8 => Ok(CastMemberType::Shape),
            9 => Ok(CastMemberType::Movie),
            10 => Ok(CastMemberType::DigitalVideo),
            11 => Ok(CastMemberType::Script),
            12 => Ok(CastMemberType::RTE),
            _ => Err(InvalidCastMemberTypeError(value)),
        }
    }
}

impl fmt::Display for CastMemberType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CastMemberType::Null => "Null",
            CastMemberType::Bitmap => "Bitmap",
            CastMemberType::FilmLoop => "FilmLoop",
            CastMemberType::Text => "Text",
            CastMemberType::Palette => "Palette",
            CastMemberType::Picture => "Picture",
            CastMemberType::Sound => "Sound",
            CastMemberType::Button => "Button",
            CastMemberType::Shape => "Shape",
            CastMemberType::Movie => "Movie",
            CastMemberType::DigitalVideo => "DigitalVideo",
            CastMemberType::Script => "Script",
            CastMemberType::RTE => "RTE",
        };
        write!(f, "{}", s)
    }
}
