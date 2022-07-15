use image::{io::Reader, RgbImage};

use crate::math_util::{clamp, Color, Point3};

use super::Texture;

pub struct ImageTexture {
    image: Option<RgbImage>,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let imagerd = Reader::open(filename);
        let image: Option<RgbImage>;
        match imagerd {
            Ok(img) => {
                if let Ok(img_decoded) = img.decode() {
                    if let Some(buf) = img_decoded.as_rgb8() {
                        image = Some(buf.clone());
                    } else {
                        image = None;
                    }
                } else {
                    image = None;
                }
            }
            Err(_) => {
                image = None;
            }
        }
        Self { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        if let Some(img) = &self.image {
            let width = img.width();
            let height = img.height();
            let u = clamp(u, 0.0, 1.0);
            let v = 1.0 - clamp(v, 0.0, 1.0);
            let i = u32::min((u * width as f64) as u32, width - 1);
            let j = u32::min((v * height as f64) as u32, height - 1);
            let color_scale = 1.0 / 255.0;
            let pixel = img.get_pixel(i, j);
            Color::new(
                color_scale * pixel[0] as f64,
                color_scale * pixel[1] as f64,
                color_scale * pixel[2] as f64,
            )
        } else {
            Color::new(0.0, 1.0, 1.0)
        }
    }
}
