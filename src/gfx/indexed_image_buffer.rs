use std::ops::{Deref, DerefMut};

use super::Palette;

/// A buffer that holds indexed image data.
///
/// Each pixel is represented by a single byte which serves as an index into a palette.
pub struct IndexedImageBuffer<Data> {
    width: usize,
    height: usize,
    data: Data,
}

impl<Data> std::fmt::Debug for IndexedImageBuffer<Data> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndexedImageBuffer")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl<Data> IndexedImageBuffer<Data>
where
    Data: Deref<Target = [u8]>,
{
    pub fn new(width: usize, height: usize, data: Data) -> Self {
        assert!(width < 0x4000);
        assert!(height < 0x4000);

        IndexedImageBuffer {
            width,
            height,
            data,
        }
    }

    pub fn new_owned(width: usize, height: usize) -> IndexedImageBuffer<Vec<u8>> {
        let data = vec![0; width * height];
        IndexedImageBuffer::new(width, height, data)
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    #[inline]
    fn index(&self, x: i16, y: i16) -> usize {
        y as usize * self.width + x as usize
    }

    #[inline]
    fn in_bounds(&self, x: i16, y: i16) -> bool {
        (x as usize) < self.width && (y as usize) < self.height
    }

    /// Returns the color index at the given coordinates or `None` if out-of-bounds.
    pub fn get_color_index(&self, x: i16, y: i16) -> Option<u8> {
        if !self.in_bounds(x, y) {
            return None;
        }

        let index = self.index(x, y);
        let data = self.get_data();

        Some(data[index])
    }

    pub fn save_to_grayscale_png(&self, path: &str) -> Result<(), png::EncodingError> {
        use png::{self, Encoder};
        use std::fs::File;
        use std::io::BufWriter;

        let file = File::create(path)?;
        let w = BufWriter::new(file);
        let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;

        writer.write_image_data(self.get_data())
    }

    pub fn save_to_png(&self, palette: &Palette, path: &str) -> Result<(), png::EncodingError> {
        use png::{self, Encoder};
        use std::fs::File;
        use std::io::BufWriter;

        let file = File::create(path)?;
        let w = BufWriter::new(file);
        let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;

        let rgba_data: Vec<u8> = self
            .get_data()
            .iter()
            .copied()
            .map(|index| palette.get_rgb888(index).unwrap_or_default())
            .flat_map(|v| [v.r, v.g, v.b, 255])
            .collect();

        writer.write_image_data(&rgba_data)
    }
}

impl<Data> IndexedImageBuffer<Data>
where
    Data: DerefMut<Target = [u8]>,
{
    pub fn get_mut_data(&mut self) -> &mut [u8] {
        self.data.as_mut()
    }

    /// Sets the color index at the given coordinates if within bounds.
    pub fn set_color_index(&mut self, x: i16, y: i16, color_index: u8) {
        if !self.in_bounds(x, y) {
            return;
        }

        let index = self.index(x, y);
        let data = self.get_mut_data();

        data[index] = color_index;
    }
}
