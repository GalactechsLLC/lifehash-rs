use crate::utils::bits::Enumerator;
use crate::{Dimensions, Pattern, Version};
use std::io::Error;

pub mod bits;
pub const fn lerp_to(to_a: f64, to_b: f64, t: f64) -> f64 {
    t * (to_b - to_a) + to_a
}
pub const fn lerp_from(from_a: f64, from_b: f64, t: f64) -> f64 {
    (from_a - t) / (from_a - from_b)
}
pub const fn lerp(from_a: f64, from_b: f64, to_c: f64, to_d: f64, t: f64) -> f64 {
    lerp_to(to_c, to_d, lerp_from(from_a, from_b, t))
}
pub const fn modulo(dividend: f64, divisor: f64) -> f64 {
    ((dividend % divisor) + divisor) % divisor
}
pub fn select_pattern(entropy: &mut Enumerator, version: Version) -> Result<Pattern, Error> {
    Ok(match version {
        Version::Fiducial | Version::GrayscaleFiducial => Pattern::Fiducial,
        _ => {
            if entropy.next_bit()? {
                Pattern::Snowflake
            } else {
                Pattern::Pinwheel
            }
        }
    })
}

pub const fn target_size(in_size: Dimensions, pattern: Pattern) -> Dimensions {
    let multiplier = match pattern {
        Pattern::Fiducial => 1,
        _ => 2,
    };
    Dimensions {
        width: in_size.width * multiplier,
        height: in_size.height * multiplier,
    }
}
