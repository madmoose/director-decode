mod bitmap;
mod cast_member;
mod cast_member_type;
mod palette;
mod script;
mod text;

use std::fmt::Display;

pub use bitmap::*;
pub use cast_member::*;
pub use cast_member_type::*;
pub use palette::*;
pub use script::*;
pub use text::*;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CastMemberId {
    id: i16,
    cast: Option<u16>,
}

impl Display for CastMemberId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(cast) = self.cast {
            write!(f, "{:5} ({:?})", self.id, cast)
        } else {
            write!(f, "{:5} (None)", self.id)
        }
    }
}

impl CastMemberId {
    pub fn new(id: i16) -> Self {
        Self { id, cast: None }
    }

    pub fn new_with_cast(id: i16, cast: u16) -> Self {
        Self {
            id,
            cast: Some(cast),
        }
    }

    pub fn id(&self) -> i16 {
        self.id
    }

    pub fn cast(&self) -> Option<u16> {
        self.cast
    }
}
