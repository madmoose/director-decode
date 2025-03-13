use std::ops::{Index, IndexMut};

use super::{Rgb888, Rgb161616};

#[derive(Debug, Clone)]
pub struct Palette([Rgb161616; 256]);

impl Default for Palette {
    fn default() -> Self {
        Self([Rgb161616::default(); 256])
    }
}

impl Palette {
    pub fn new() -> Self {
        Palette::default()
    }

    pub fn set<C: Into<Rgb161616>>(&mut self, index: u8, color: C) {
        self.0[index as usize] = color.into()
    }

    pub fn get_rgb888(&self, index: u8) -> Option<Rgb888> {
        self.0.get(index as usize).map(Rgb888::from)
    }
}

impl Index<usize> for Palette {
    type Output = Rgb161616;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Palette {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
