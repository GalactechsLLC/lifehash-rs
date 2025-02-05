use crate::colors::rgb;
use crate::utils::modulo;
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
        let brightness = self.brightness.clamp(0.0, 1.0);
        let saturation = self.saturation.clamp(0.0, 1.0);
        let red;
        let green;
        let blue;
        if saturation <= 0.0 {
            red = brightness;
            green = brightness;
            blue = brightness;
        } else {
            let mut hue = modulo(self.hue, 1.0);
            if hue < 0.0 {
                hue += 1.0;
            }
            hue *= 6.0;
            let hue_floor = hue.floor() as i32;
            let hue_remainder = hue - f64::from(hue_floor);
            match hue_floor {
                0 => {
                    red = brightness;
                    green = brightness * (1.0 - saturation * (1.0 - hue_remainder));
                    blue = brightness * (1.0 - saturation);
                }
                1 => {
                    red = brightness * (1.0 - saturation * hue_remainder);
                    green = brightness;
                    blue = brightness * (1.0 - saturation);
                }
                2 => {
                    red = brightness * (1.0 - saturation);
                    green = brightness;
                    blue = brightness * (1.0 - saturation * (1.0 - hue_remainder));
                }
                3 => {
                    red = brightness * (1.0 - saturation);
                    green = brightness * (1.0 - saturation * hue_remainder);
                    blue = brightness;
                }
                4 => {
                    red = brightness * (1.0 - saturation * (1.0 - hue_remainder));
                    green = brightness * (1.0 - saturation);
                    blue = brightness;
                }
                5 => {
                    red = brightness;
                    green = brightness * (1.0 - saturation);
                    blue = brightness * (1.0 - saturation * hue_remainder);
                }
                _ => {
                    return Err(Error::new(ErrorKind::InvalidData, "Invalid HSB color."));
                }
            }
        }
        Ok(rgb::Color::new(red, green, blue))
    }
}
