use super::Pos;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Rect {
    pub y0: i16,
    pub x0: i16,
    pub y1: i16,
    pub x1: i16,
}

impl Rect {
    pub fn new(y0: i16, x0: i16, y1: i16, x1: i16) -> Self {
        Self { y0, x0, y1, x1 }
    }

    pub fn width(&self) -> i16 {
        self.x1 - self.x0
    }

    pub fn height(&self) -> i16 {
        self.y1 - self.y0
    }

    pub fn is_empty(&self) -> bool {
        self.width() <= 0 || self.height() <= 0
    }

    pub fn scale(self, scale: f32) -> Self {
        Self {
            y0: (scale * self.y0 as f32) as i16,
            x0: (scale * self.x0 as f32) as i16,
            y1: (scale * self.y1 as f32) as i16, // + (scale * self.height() as f32) as i16,
            x1: (scale * self.x1 as f32) as i16, // + (scale * self.width() as f32) as i16,
        }
    }

    pub fn translate<P: Into<Pos>>(self, pos: P) -> Self {
        let pos: Pos = pos.into();
        Self {
            y0: self.y0 + pos.y,
            x0: self.x0 + pos.x,
            y1: self.y1 + pos.y,
            x1: self.x1 + pos.x,
        }
    }
}
