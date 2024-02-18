use etheris_common::Color;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, Rgba, RgbaImage};
use rayon::prelude::{ParallelBridge, ParallelIterator};

struct BoundingBox {
    min_x: u32,
    max_x: u32,
    min_y: u32,
    max_y: u32,
}

pub fn trim_transparent_pixels(image: DynamicImage) -> DynamicImage {
    let (width, height) = image.dimensions();
    let mut bbox = BoundingBox {
        min_x: width,
        max_x: 0,
        min_y: height,
        max_y: 0,
    };

    for (x, y, pixel) in image.pixels() {
        if pixel[3] != 0 {
            bbox.min_x = bbox.min_x.min(x);
            bbox.max_x = bbox.max_x.max(x);
            bbox.min_y = bbox.min_y.min(y);
            bbox.max_y = bbox.max_y.max(y);
        }
    }

    let trimmed_width = bbox.max_x - bbox.min_x + 1;
    let trimmed_height = bbox.max_y - bbox.min_y + 1;

    let trimmed_image = ImageBuffer::from_fn(trimmed_width, trimmed_height, |x, y| {
        let pixel = image.get_pixel(x + bbox.min_x, y + bbox.min_y);
        Rgba(pixel.0)
    });

    trimmed_image.into()
}

pub fn remove_chroma_key(image: &mut RgbaImage) {
    let chroma_key_color = Rgba([0, 255, 0, 255]);

    for pixel in image.pixels_mut() {
        if *pixel == chroma_key_color {
            *pixel = Rgba([0, 0, 0, 0]);
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Tint {
    pub color: Color,
    pub factor: f32,
    pub brightness_threshold: Option<u8>,
}

pub fn tint_image(image: &mut RgbaImage, tint: Tint) {
    let Rgb([tint_r, tint_g, tint_b]) = tint.color.to_rgb();
    let factor = tint.factor;

    image.pixels_mut().par_bridge().for_each(|pixel| {
        let Rgba([r, g, b, a]) = *pixel;
        // Ignore transparent pixels
        if a < 200 {
            return;
        }

        let mut factor = factor;
        if let Some(brightness_threshold) = tint.brightness_threshold {
            let brightness =
                ((0.2126 * r as f32) + (0.7152 * g as f32) + (0.0722 * b as f32)) as u8;
            factor = if brightness > brightness_threshold {
                factor * 0.9
            } else {
                factor
            };
        }

        let r_new = tint_color_by_factor(factor, r as f32, tint_r);
        let g_new = tint_color_by_factor(factor, g as f32, tint_g);
        let b_new = tint_color_by_factor(factor, b as f32, tint_b);

        *pixel = Rgba([r_new, g_new, b_new, a]);
    });
}

#[inline(always)]
fn tint_color_by_factor(factor: f32, a: f32, b: f32) -> u8 {
    ((factor * a * b) + ((1.0 - factor) * a)) as u8
}
