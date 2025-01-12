use crate::colors::{BLACK, WHITE};
use crate::utils::clamped;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
    pub const fn new_u8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0,
        }
    }
    pub fn luminance(&self) -> f64 {
        ((0.299 * self.r).powf(2.0) + (0.587 * self.g).powf(2.0) + (0.114 * self.b).powf(2.0))
            .sqrt()
    }
    pub fn burn(&self, t: f64) -> Color {
        let f = (1.0 - t).max(1.0e-7);
        Color::new(
            (1.0 - (1.0 - self.r) / f).min(1.0),
            (1.0 - (1.0 - self.g) / f).min(1.0),
            (1.0 - (1.0 - self.b) / f).min(1.0),
        )
    }
    pub fn lerp_to(&self, other: &Color, t: f64) -> Color {
        let f = clamped(t);
        let red = clamped(self.r * (1.0 - f) + other.r * f);
        let green = clamped(self.g * (1.0 - f) + other.g * f);
        let blue = clamped(self.b * (1.0 - f) + other.b * f);
        Color::new(red, green, blue)
    }
    pub fn lighten(&self, t: f64) -> Color {
        self.lerp_to(&WHITE, t)
    }
    pub fn darken(&self, t: f64) -> Color {
        self.lerp_to(&BLACK, t)
    }
}
impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}
