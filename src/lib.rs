extern crate core;

use crate::colors::rgb::Color;
use crate::lifehash::{from_data, from_digest};
use clap::Parser;
use png::ColorType;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Error, ErrorKind, Read};
use std::path::{Path, PathBuf};

pub mod colors;
mod grids;
pub mod lifehash;
pub mod utils;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Point {
    pub x: usize,
    pub y: usize,
}

trait ColorValues<T> {
    fn color_for_value(value: &T) -> Color;
}

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
            3 => Version::Detailed,
            4 => Version::Fiducial,
            5 => Version::GrayscaleFiducial,
            _ => Version::Version2,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Pattern {
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
    } else {
        //Default to RGB if no Alpha, Grayscale uses 3 Channels
        png.set_color(ColorType::Rgb);
    }
    let mut writer = png.write_header()?;
    writer.write_image_data(&bitmap.pixels)?;
    Ok(writer.finish()?)
}

pub enum Either<T, U> {
    Left(T),
    Right(U),
}

pub struct RuntimeArgs {
    pub module_size: u8,
    pub use_alpha: bool,
    pub version: Version,
    pub input: Either<String, Vec<u8>>,
    pub output_file: PathBuf,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'i', long = "in-file", value_name = "FILE")]
    pub input_file: Option<PathBuf>,
    #[arg(short = 'x', long = "hex")]
    pub hex: Option<String>,
    #[arg(short = 'o', long = "out-file", value_name = "FILE")]
    pub output_file: Option<PathBuf>,
    #[arg(short = 'm', long = "module-size")]
    pub module_size: Option<u8>,
    #[arg(short = 'a', long = "use-alpha")]
    pub use_alpha: Option<bool>,
    #[arg(short = 'v', long = "lh-version")]
    pub lh_version: Option<u8>,
}

pub fn parse_cli_from<I, T>(itr: I) -> Result<RuntimeArgs, Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let cli = Cli::parse_from(itr);
    Ok(RuntimeArgs {
        module_size: cli.module_size.unwrap_or(1),
        use_alpha: cli.use_alpha.unwrap_or(false),
        version: cli.lh_version.map_or(Version::Version2, Version::from),
        input: parse_input(&cli)?,
        output_file: cli
            .output_file
            .unwrap_or_else(|| PathBuf::from("./lifehash.png")),
    })
}

#[cfg(not(tarpaulin_include))]
pub fn parse_cli() -> Result<RuntimeArgs, Error> {
    parse_cli_from(std::env::args())
}

fn parse_input(cli: &Cli) -> Result<Either<String, Vec<u8>>, Error> {
    if let Some(input) = &cli.input_file {
        let mut buf = String::new();
        let mut file = File::open(input)?;
        file.read_to_string(&mut buf)?;
        Ok(Either::Left(buf))
    } else if let Some(hex) = &cli.hex {
        hex::decode(hex)
            .map(Either::Right)
            .map_err(|e| Error::new(ErrorKind::InvalidData, format!("failed to decode hex: {e}")))
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        Ok(Either::Left(buf))
    }
}

pub fn run(args: RuntimeArgs) -> Result<(), Error> {
    let (image, _) = match args.input {
        Either::Left(input) => from_data(
            input.as_bytes(),
            args.version,
            args.module_size as usize,
            args.use_alpha,
        )?,
        Either::Right(input) => from_digest(
            &input,
            args.version,
            args.module_size as usize,
            args.use_alpha,
        )?,
    };
    save_image(&image, &args.output_file)
}
