use std::ops::{Deref, DerefMut};

use super::rgb888::Rgb888;

pub struct ImageBuffer<Data> {
    width: usize,
    height: usize,
    data: Data,
}

impl<Data> ImageBuffer<Data>
where
    Data: Deref<Target = [u32]>,
{
    pub fn new(width: usize, height: usize, data: Data) -> Self {
        assert!(width < 0x4000);
        assert!(height < 0x4000);

        ImageBuffer {
            width,
            height,
            data,
        }
    }

    pub fn new_owned(width: usize, height: usize) -> ImageBuffer<Vec<u32>> {
        let data = vec![0; width * height];
        ImageBuffer::new(width, height, data)
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get_data(&self) -> &[u32] {
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

    pub fn get_pixel(&self, x: i16, y: i16) -> Option<Rgb888> {
        if !self.in_bounds(x, y) {
            return None;
        }

        let index = self.index(x, y);
        let data = self.get_data();

        Some(Rgb888::from_u32(data[index]))
    }

    pub fn save_to_png(&self, path: &str) -> Result<(), png::EncodingError> {
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
            .map(|v| v.to_be_bytes())
            .flat_map(|v| [v[1], v[2], v[3], v[0]])
            .collect();

        writer.write_image_data(&rgba_data)
    }
}

impl<Data> ImageBuffer<Data>
where
    Data: DerefMut<Target = [u32]>,
{
    fn get_mut_data(&mut self) -> &mut [u32] {
        self.data.as_mut()
    }

    pub fn set_pixel(&mut self, x: i16, y: i16, color: Rgb888) {
        if !self.in_bounds(x, y) {
            return;
        }

        let index = self.index(x, y);
        let data = self.get_mut_data();

        data[index] = color.to_u32();
    }
}
