#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Size {
    pub h: i16,
    pub w: i16,
}

impl Size {
    pub fn new(h: i16, w: i16) -> Self {
        Self { h, w }
    }

    pub fn is_empty(&self) -> bool {
        self.h <= 0 || self.w <= 0
    }
}
