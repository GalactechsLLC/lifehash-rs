use crate::colors::gradient::make_hue;
use crate::colors::{rgb::Color, BLACK};
use crate::utils::modulo;

pub enum ColorFunction<'a> {
    Blend(Color, Color),
    BlendRef(&'a [Color]),
    BlendVec(Vec<Color>),
    Reverse(Box<ColorFunction<'a>>),
    MakeHue,
}
impl ColorFunction<'_> {
    pub fn apply(&self, t: f64) -> Color {
        match self {
            ColorFunction::Blend(color1, color2) => color1.lerp_to(color2, t),
            ColorFunction::BlendVec(colors) => blend_vals(t, colors.as_slice()),
            ColorFunction::BlendRef(colors) => blend_vals(t, colors),
            ColorFunction::Reverse(func) => func.apply(1.0 - t),
            ColorFunction::MakeHue => {
                make_hue(t).expect("Expected known HSB color to convert to RGB")
            }
        }
    }
}
fn blend_vals(t: f64, colors: &[Color]) -> Color {
    if colors.is_empty() {
        ColorFunction::Blend(BLACK, BLACK).apply(t)
    } else if colors.len() == 1 {
        ColorFunction::Blend(colors[0], colors[0]).apply(t)
    } else if colors.len() == 2 {
        ColorFunction::Blend(colors[0], colors[1]).apply(t)
    } else if t >= 1.0 {
        colors[colors.len() - 1]
    } else if t <= 0.0 {
        colors[0]
    } else {
        let segments = colors.len() - 1;
        let s = t * segments as f64;
        let segment = s as usize;
        let segment_frac = modulo(s, 1.0);
        let c1 = colors[segment];
        let c2 = colors[segment + 1];
        c1.lerp_to(&c2, segment_frac)
    }
}
