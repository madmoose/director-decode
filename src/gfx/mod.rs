mod image_buffer;
mod indexed_image_buffer;
mod palette;
mod pos;
mod rect;
mod rgb161616;
mod rgb888;
mod size;

use std::ops::DerefMut;

pub use image_buffer::*;
pub use indexed_image_buffer::*;
pub use palette::*;
pub use pos::*;
pub use rect::*;
pub use rgb888::*;
pub use rgb161616::*;
pub use size::*;

pub type Image = ImageBuffer<Vec<u8>>;
pub type IndexedImage = IndexedImageBuffer<Vec<u8>>;

pub fn blit<ToData: DerefMut<Target = [u32]>, FromData: DerefMut<Target = [u8]>>(
    destination: &mut ImageBuffer<ToData>,
    destination_rect: Rect,
    source: &IndexedImageBuffer<FromData>,
    source_rect: Rect,
    palette: &Palette,
    transparent_color_index: Option<u8>,
) {
    if destination_rect.is_empty() || source_rect.is_empty() {
        return;
    }

    let x_scale = source_rect.width() as f32 / destination_rect.width() as f32;
    let y_scale = source_rect.height() as f32 / destination_rect.height() as f32;

    for dy in 0..destination_rect.height() {
        for dx in 0..destination_rect.width() {
            let sx = (dx as f32 * x_scale) as i16 + source_rect.x0;
            let sy = (dy as f32 * y_scale) as i16 + source_rect.y0;

            let Some(color_index) = source.get_color_index(sx, sy) else {
                continue;
            };

            if transparent_color_index.is_some_and(|v| v == color_index) {
                continue;
            }

            let Some(color) = palette.get_rgb888(color_index) else {
                continue;
            };

            destination.set_pixel(destination_rect.x0 + dx, destination_rect.y0 + dy, color);
        }
    }
}
