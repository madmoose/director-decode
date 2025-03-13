use std::ops::Neg;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Pos {
    pub y: i16,
    pub x: i16,
}

impl Neg for Pos {
    type Output = Pos;

    fn neg(self) -> Self::Output {
        Self {
            y: -self.y,
            x: -self.x,
        }
    }
}
