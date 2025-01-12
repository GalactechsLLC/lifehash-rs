use crate::colors::functions::ColorFunction;
use crate::colors::{
    hsb::Color as HSBColor, rgb::Color, BLACK, GRAYSCALE, SPECTRUM, SPECTRUM_CMYK_SAFE, WHITE,
};
use crate::utils::bits::Enumerator;
use crate::utils::{lerp, modulo};
use crate::Version;
use std::cmp::Ordering;
use std::io::{Error, ErrorKind};

pub fn select_grayscale<'a>(entropy: &mut Enumerator) -> Result<ColorFunction<'a>, Error> {
    Ok(if entropy.next()? {
        GRAYSCALE
    } else {
        ColorFunction::Reverse(Box::new(GRAYSCALE))
    })
}

pub fn make_hue(t: f64) -> Result<Color, Error> {
    HSBColor::new(t, 1.0, 1.0).rgb()
}

pub fn adjust_for_luminance(color: Color, contrast_color: Color) -> Color {
    let lum = color.luminance();
    let contrast_lum = contrast_color.luminance();
    let threshold = 0.6;
    let offset = (lum - contrast_lum).abs();
    if offset > threshold {
        return color;
    }
    let boost = 0.7;
    let t = lerp(0.0, threshold, boost, 0.0, offset);
    if contrast_lum > lum {
        // darken this color
        color.darken(t).burn(t * 0.6)
    } else {
        // lighten this color
        color.lighten(t).burn(t * 0.6)
    }
}

pub fn monochromatic<'a>(
    entropy: &mut Enumerator,
    hue_generator: ColorFunction<'a>,
) -> Result<ColorFunction<'a>, Error> {
    let hue = entropy.next_frac()?;
    let is_tint = entropy.next()?;
    let is_reversed = entropy.next()?;
    let key_advance = entropy.next_frac()? * 0.3 + 0.05;
    let neutral_advance = entropy.next_frac()? * 0.3 + 0.05;
    let mut key_color = hue_generator.apply(hue);
    let contrast_brightness;
    if is_tint {
        contrast_brightness = 1.0;
        key_color = key_color.darken(0.5);
    } else {
        contrast_brightness = 0.0;
    }
    let neutral_color = GRAYSCALE.apply(contrast_brightness);
    let key_color_2 = key_color.lerp_to(&neutral_color, key_advance);
    let neutral_color_2 = neutral_color.lerp_to(&key_color, neutral_advance);
    let gradient = ColorFunction::Blend(key_color_2, neutral_color_2);
    Ok(if is_reversed {
        ColorFunction::Reverse(Box::from(gradient))
    } else {
        gradient
    })
}

pub fn monochromatic_fiducial<'a>(entropy: &mut Enumerator) -> Result<ColorFunction<'a>, Error> {
    let hue = entropy.next_frac()?;
    let is_reversed = entropy.next()?;
    let is_tint = entropy.next()?;
    let contrast_color = if is_tint { WHITE } else { BLACK };
    let key_color = adjust_for_luminance(SPECTRUM_CMYK_SAFE.apply(hue), contrast_color);
    let gradient = ColorFunction::BlendVec(vec![key_color, contrast_color, key_color]);
    Ok(if is_reversed {
        ColorFunction::Reverse(Box::from(gradient))
    } else {
        gradient
    })
}

pub fn complementary<'a>(
    entropy: &mut Enumerator,
    hue_generator: ColorFunction<'a>,
) -> Result<ColorFunction<'a>, Error> {
    let spectrum1 = entropy.next_frac()?;
    let spectrum2 = modulo(spectrum1 + 0.5, 1.0);
    let lighter_advance = entropy.next_frac()? * 0.3;
    let darker_advance = entropy.next_frac()? * 0.3;
    let is_reversed = entropy.next()?;
    let color1 = hue_generator.apply(spectrum1);
    let color2 = hue_generator.apply(spectrum2);
    let luma1 = color1.luminance();
    let luma2 = color2.luminance();
    let darker_color;
    let lighter_color;
    if luma1 > luma2 {
        darker_color = color2;
        lighter_color = color1;
    } else {
        darker_color = color1;
        lighter_color = color2;
    }
    let adjusted_lighter_color = lighter_color.lighten(lighter_advance);
    let adjusted_darker_color = darker_color.darken(darker_advance);
    let gradient = ColorFunction::Blend(adjusted_darker_color, adjusted_lighter_color);
    Ok(if is_reversed {
        ColorFunction::Reverse(Box::from(gradient))
    } else {
        gradient
    })
}

pub fn complementary_fiducial<'a>(entropy: &mut Enumerator) -> Result<ColorFunction<'a>, Error> {
    let spectrum1 = entropy.next_frac()?;
    let spectrum2 = modulo(spectrum1 + 0.5, 1.0);
    let is_tint = entropy.next()?;
    let is_reversed = entropy.next()?;
    let neutral_color_bias = entropy.next()?;
    let neutral_color = if is_tint { WHITE } else { BLACK };
    let color1 = SPECTRUM_CMYK_SAFE.apply(spectrum1);
    let color2 = SPECTRUM_CMYK_SAFE.apply(spectrum2);
    let biased_neutral_color = neutral_color
        .lerp_to(&if neutral_color_bias { color1 } else { color2 }, 0.2)
        .burn(0.1);
    let gradient = ColorFunction::BlendVec(vec![
        adjust_for_luminance(color1, biased_neutral_color),
        biased_neutral_color,
        adjust_for_luminance(color2, biased_neutral_color),
    ]);
    Ok(if is_reversed {
        ColorFunction::Reverse(Box::from(gradient))
    } else {
        gradient
    })
}

pub fn triadic<'a>(
    entropy: &mut Enumerator,
    hue_generator: ColorFunction<'a>,
) -> Result<ColorFunction<'a>, Error> {
    let spectrum1 = entropy.next_frac()?;
    let spectrum2 = modulo(spectrum1 + 1.0 / 3.0, 1.0);
    let spectrum3 = modulo(spectrum1 + 2.0 / 3.0, 1.0);
    let lighter_advance = entropy.next_frac()? * 0.3;
    let darker_advance = entropy.next_frac()? * 0.3;
    let is_reversed = entropy.next()?;
    let color1 = hue_generator.apply(spectrum1);
    let color2 = hue_generator.apply(spectrum2);
    let color3 = hue_generator.apply(spectrum3);
    let mut colors = [color1, color2, color3];
    colors.sort_by(|a, b| {
        if a.luminance() < b.luminance() {
            Ordering::Less
        } else if a.luminance() > b.luminance() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    let darker_color = colors[0];
    let middle_color = colors[1];
    let lighter_color = colors[2];
    let adjusted_lighter_color = lighter_color.lighten(lighter_advance);
    let adjusted_darker_color = darker_color.darken(darker_advance);
    let vals = vec![adjusted_lighter_color, middle_color, adjusted_darker_color];
    let gradient = ColorFunction::BlendVec(vals);
    Ok(if is_reversed {
        ColorFunction::Reverse(Box::from(gradient))
    } else {
        gradient
    })
}

pub fn triadic_fiducial<'a>(entropy: &mut Enumerator) -> Result<ColorFunction<'a>, Error> {
    let spectrum1 = entropy.next_frac()?;
    let spectrum2 = modulo(spectrum1 + 1.0 / 3.0, 1.0);
    let spectrum3 = modulo(spectrum1 + 2.0 / 3.0, 1.0);
    let is_tint = entropy.next()?;
    let neutral_insert_index = entropy.next_u8()? % 2 + 1;
    let is_reversed = entropy.next()?;
    let neutral_color = if is_tint { WHITE } else { BLACK };
    let mut colors = vec![
        SPECTRUM_CMYK_SAFE.apply(spectrum1),
        SPECTRUM_CMYK_SAFE.apply(spectrum2),
        SPECTRUM_CMYK_SAFE.apply(spectrum3),
    ];
    match neutral_insert_index {
        1 => {
            colors[0] = adjust_for_luminance(colors[0], neutral_color);
            colors[1] = adjust_for_luminance(colors[1], neutral_color);
            colors[2] = adjust_for_luminance(colors[2], colors[1]);
        }
        2 => {
            colors[1] = adjust_for_luminance(colors[1], neutral_color);
            colors[2] = adjust_for_luminance(colors[2], neutral_color);
            colors[0] = adjust_for_luminance(colors[0], colors[1]);
        }
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid Neutral Insert Index",
            ))
        }
    }
    colors.insert(neutral_insert_index, neutral_color);
    let gradient = ColorFunction::BlendVec(colors);
    Ok(if is_reversed {
        ColorFunction::Reverse(Box::from(gradient))
    } else {
        gradient
    })
}

pub fn analogous<'a>(
    entropy: &mut Enumerator,
    hue_generator: ColorFunction<'a>,
) -> Result<ColorFunction<'a>, Error> {
    let spectrum1 = entropy.next_frac()?;
    let spectrum2 = modulo(spectrum1 + 1.0 / 12.0, 1.0);
    let spectrum3 = modulo(spectrum1 + 2.0 / 12.0, 1.0);
    let spectrum4 = modulo(spectrum1 + 3.0 / 12.0, 1.0);
    let advance = entropy.next_frac()? * 0.5 + 0.2;
    let is_reversed = entropy.next()?;
    let color1 = hue_generator.apply(spectrum1);
    let color2 = hue_generator.apply(spectrum2);
    let color3 = hue_generator.apply(spectrum3);
    let color4 = hue_generator.apply(spectrum4);
    let darkest_color;
    let dark_color;
    let light_color;
    let lightest_color;
    if color1.luminance() < color4.luminance() {
        darkest_color = color1;
        dark_color = color2;
        light_color = color3;
        lightest_color = color4;
    } else {
        darkest_color = color4;
        dark_color = color3;
        light_color = color2;
        lightest_color = color1;
    }
    let adjusted_darkest_color = darkest_color.darken(advance);
    let adjusted_dark_color = dark_color.darken(advance / 2.0);
    let adjusted_light_color = light_color.lighten(advance / 2.0);
    let adjusted_lightest_color = lightest_color.lighten(advance);
    let gradient = ColorFunction::BlendVec(vec![
        adjusted_darkest_color,
        adjusted_dark_color,
        adjusted_light_color,
        adjusted_lightest_color,
    ]);
    Ok(if is_reversed {
        ColorFunction::Reverse(Box::from(gradient))
    } else {
        gradient
    })
}

pub fn analogous_fiducial<'a>(entropy: &mut Enumerator) -> Result<ColorFunction<'a>, Error> {
    let spectrum1 = entropy.next_frac()?;
    let spectrum2 = modulo(spectrum1 + 1.0 / 10.0, 1.0);
    let spectrum3 = modulo(spectrum1 + 2.0 / 10.0, 1.0);
    let is_tint = entropy.next()?;
    let neutral_insert_index = entropy.next_u8()? % 2 + 1;
    let is_reversed = entropy.next()?;
    let neutral_color = if is_tint { WHITE } else { BLACK };
    let mut colors = vec![
        SPECTRUM_CMYK_SAFE.apply(spectrum1),
        SPECTRUM_CMYK_SAFE.apply(spectrum2),
        SPECTRUM_CMYK_SAFE.apply(spectrum3),
    ];
    match neutral_insert_index {
        1 => {
            colors[0] = adjust_for_luminance(colors[0], neutral_color);
            colors[1] = adjust_for_luminance(colors[1], neutral_color);
            colors[2] = adjust_for_luminance(colors[2], colors[1]);
        }
        2 => {
            colors[1] = adjust_for_luminance(colors[1], neutral_color);
            colors[2] = adjust_for_luminance(colors[2], neutral_color);
            colors[0] = adjust_for_luminance(colors[0], colors[1]);
        }
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid Neutral Insert Index",
            ))
        }
    }
    colors.insert(neutral_insert_index, neutral_color);
    let gradient = ColorFunction::BlendVec(colors);
    Ok(if is_reversed {
        ColorFunction::Reverse(Box::new(gradient))
    } else {
        gradient
    })
}

pub fn select_gradient<'a>(
    entropy: &mut Enumerator,
    version: Version,
) -> Result<ColorFunction<'a>, Error> {
    if version == Version::GrayscaleFiducial {
        return select_grayscale(entropy);
    }
    let value = entropy.next_u2()?;
    match value {
        0 => match version {
            Version::Version1 => monochromatic(entropy, ColorFunction::MakeHue),
            Version::Version2 | Version::Detailed => monochromatic(entropy, SPECTRUM_CMYK_SAFE),
            Version::Fiducial => monochromatic_fiducial(entropy),
            Version::GrayscaleFiducial => Ok(GRAYSCALE),
        },
        1 => match version {
            Version::Version1 => complementary(entropy, SPECTRUM),
            Version::Version2 | Version::Detailed => complementary(entropy, SPECTRUM_CMYK_SAFE),
            Version::Fiducial => complementary_fiducial(entropy),
            Version::GrayscaleFiducial => Ok(GRAYSCALE),
        },
        2 => match version {
            Version::Version1 => triadic(entropy, SPECTRUM),
            Version::Version2 | Version::Detailed => triadic(entropy, SPECTRUM_CMYK_SAFE),
            Version::Fiducial => triadic_fiducial(entropy),
            Version::GrayscaleFiducial => Ok(GRAYSCALE),
        },
        3 => match version {
            Version::Version1 => analogous(entropy, SPECTRUM),
            Version::Version2 | Version::Detailed => analogous(entropy, SPECTRUM_CMYK_SAFE),
            Version::Fiducial => analogous_fiducial(entropy),
            Version::GrayscaleFiducial => Ok(GRAYSCALE),
        },
        _ => Ok(GRAYSCALE),
    }
}
