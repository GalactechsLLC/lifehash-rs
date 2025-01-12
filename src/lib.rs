extern crate core;

use crate::colors::rgb::Color;
use png::ColorType;
use std::fs::File;
use std::io::{BufWriter, Error, ErrorKind};
use std::path::Path;

mod colors;
mod grids;
pub mod lifehash;
mod utils;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Dimensions {
    pub width: i32,
    pub height: i32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

trait ColorValues<T> {
    fn color_for_value(value: &T) -> Color;
}

const ZERO: Point = Point { x: 0, y: 0 };

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Version {
    Version1, // DEPRECATED. Uses HSB gamut. Not CMYK-friendly. Has some minor gradient bugs.
    Version2, // CMYK-friendly gamut. Recommended for most purposes.
    Detailed, // Double resolution. CMYK-friendly gamut gamut.
    Fiducial, // Optimized for generating machine-vision fiducials. High-contrast. CMYK-friendly gamut.
    GrayscaleFiducial, // Optimized for generating machine-vision fiducials. High-contrast.
}
impl From<u8> for Version {
    fn from(version: u8) -> Self {
        match version {
            1 => Version::Version1,
            2 => Version::Version2,
            3 => Version::Detailed,
            4 => Version::Fiducial,
            5 => Version::GrayscaleFiducial,
            _ => Version::Version2,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Pattern {
    Snowflake, // Mirror around central axes.
    Pinwheel,  // Rotate around center.
    Fiducial,  // Identity.
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub channels: usize,
    pub pixels: Vec<u8>,
}

pub fn save_image(bitmap: &Image, filename: &Path) -> Result<(), Error> {
    let file = File::create(filename)?;
    let buffer = BufWriter::new(file);
    let mut png = png::Encoder::new(buffer, bitmap.width as u32, bitmap.height as u32);
    if bitmap.channels == 4 {
        png.set_color(ColorType::Rgba);
    } else if bitmap.channels == 3 {
        png.set_color(ColorType::Rgb);
    } else {
        png.set_color(ColorType::Grayscale);
    }
    let mut writer = png.write_header()?;
    writer.write_image_data(&bitmap.pixels).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed to write image data: {}", e),
        )
    })?;
    writer.finish().map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed to finish image data: {}", e),
        )
    })
}
