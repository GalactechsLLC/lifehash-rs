use crate::colors::rgb;
use crate::utils::{clamped, modulo};
use std::io::{Error, ErrorKind};

pub struct Color {
    hue: f64,
    saturation: f64,
    brightness: f64,
}
impl Color {
    pub fn new(hue: f64, saturation: f64, brightness: f64) -> Self {
        Self {
            hue,
            saturation,
            brightness,
        }
    }
    pub fn rgb(&self) -> Result<rgb::Color, Error> {
        let v = clamped(self.brightness);
        let s = clamped(self.saturation);
        let red;
        let green;
        let blue;
        if s <= 0.0 {
            red = v;
            green = v;
            blue = v;
        } else {
            let mut h = modulo(self.hue, 1.0);
            if h < 0.0 {
                h += 1.0;
            }
            h *= 6.0;
            let i = h.floor() as i32;
            let f = h - i as f64;
            let p = v * (1.0 - s);
            let q = v * (1.0 - s * f);
            let t = v * (1.0 - s * (1.0 - f));
            match i {
                0 => {
                    red = v;
                    green = t;
                    blue = p;
                }
                1 => {
                    red = q;
                    green = v;
                    blue = p;
                }
                2 => {
                    red = p;
                    green = v;
                    blue = t;
                }
                3 => {
                    red = p;
                    green = q;
                    blue = v;
                }
                4 => {
                    red = t;
                    green = p;
                    blue = v;
                }
                5 => {
                    red = v;
                    green = p;
                    blue = q;
                }
                _ => {
                    return Err(Error::new(ErrorKind::InvalidData, "Invalid HSB color."));
                }
            }
        }
        Ok(rgb::Color::new(red, green, blue))
    }
}
