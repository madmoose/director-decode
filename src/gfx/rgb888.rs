use super::rgb161616::Rgb161616;

#[derive(Debug, Default, Copy, Clone)]
pub struct Rgb888 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb888 {
    /// Converts the `Rgb888` color to a 32-bit unsigned integer in xRGB format.
    /// The alpha channel is set to 255 (fully opaque).
    pub fn to_u32(self) -> u32 {
        0xff000000 | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Converts a 32-bit unsigned integer in xRGB format to an `Rgb888` color.
    pub fn from_u32(value: u32) -> Self {
        Rgb888 {
            r: ((value >> 16) & 0xFF) as u8,
            g: ((value >> 8) & 0xFF) as u8,
            b: (value & 0xFF) as u8,
        }
    }
}

impl From<Rgb161616> for Rgb888 {
    fn from(value: Rgb161616) -> Self {
        Self {
            r: (value.r >> 8) as u8,
            g: (value.g >> 8) as u8,
            b: (value.b >> 8) as u8,
        }
    }
}

impl From<&Rgb161616> for Rgb888 {
    fn from(value: &Rgb161616) -> Self {
        Rgb888::from(*value)
    }
}

impl From<u32> for Rgb888 {
    fn from(value: u32) -> Self {
        Rgb888::from_u32(value)
    }
}
