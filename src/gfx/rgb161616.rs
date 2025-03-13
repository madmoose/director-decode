use super::Rgb888;

#[derive(Debug, Default, Copy, Clone)]
pub struct Rgb161616 {
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

impl From<Rgb888> for Rgb161616 {
    fn from(value: Rgb888) -> Self {
        Self {
            r: (value.r as u16) << 8,
            g: (value.g as u16) << 8,
            b: (value.b as u16) << 8,
        }
    }
}

impl From<(u8, u8, u8)> for Rgb161616 {
    fn from(value: (u8, u8, u8)) -> Self {
        Self {
            r: (value.0 as u16) << 8,
            g: (value.1 as u16) << 8,
            b: (value.2 as u16) << 8,
        }
    }
}

impl From<(u16, u16, u16)> for Rgb161616 {
    fn from(value: (u16, u16, u16)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}
