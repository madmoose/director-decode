mod chunks;
mod projector;
mod riff_file;
mod version;
mod vlist;

pub mod cast_members;
pub mod tags;

pub use projector::Projector;
pub use riff_file::RiffFile;

use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy)]
pub enum Tempo {
    None,
    WaitForMouse,
    WaitForSoundChannel1,
    WaitForSoundChannel2,
    FPS(u8),
}

impl Default for Tempo {
    fn default() -> Self {
        Tempo::FPS(1)
    }
}

#[derive(Debug)]
pub struct InvalidTempoError(i8);

impl fmt::Display for InvalidTempoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid value for tempo: {}", self.0)
    }
}

impl Error for InvalidTempoError {}

impl TryFrom<i8> for Tempo {
    type Error = InvalidTempoError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tempo::None),
            1.. => Ok(Tempo::FPS(value as u8)),
            -121 => Ok(Tempo::WaitForSoundChannel1),
            -122 => Ok(Tempo::WaitForSoundChannel2),
            -128 => Ok(Tempo::WaitForMouse),
            _ => Err(InvalidTempoError(value)),
        }
    }
}
