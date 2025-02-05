use crate::colors::functions::ColorFunction;
use crate::colors::rgb::Color;

pub mod functions;
pub mod gradient;
pub mod hsb;
pub mod rgb;
pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};
pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};
pub const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
};
pub const BLUE: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 1.0,
};
pub const SPECTRUM: ColorFunction = ColorFunction::BlendRef(&[
    Color::new_u8(0, 168, 222),
    Color::new_u8(51, 51, 145),
    Color::new_u8(233, 19, 136),
    Color::new_u8(235, 45, 46),
    Color::new_u8(253, 233, 43),
    Color::new_u8(0, 158, 84),
    Color::new_u8(0, 168, 222),
]);

pub const SPECTRUM_CMYK_SAFE: ColorFunction = ColorFunction::BlendRef(&[
    Color::new_u8(0, 168, 222),
    Color::new_u8(41, 60, 130),
    Color::new_u8(210, 59, 130),
    Color::new_u8(217, 63, 53),
    Color::new_u8(244, 228, 81),
    Color::new_u8(0, 158, 84),
    Color::new_u8(0, 168, 222),
]);
pub const GRAYSCALE: ColorFunction = ColorFunction::Blend(BLACK, WHITE);
