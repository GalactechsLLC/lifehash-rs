use clap::Parser;
use lifehash_lib::lifehash::{from_data, from_digest};
use lifehash_lib::{save_image, Version};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
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

fn main() {
    let cli = Cli::parse();
    let module_size = cli.module_size.unwrap_or(1);
    let use_alpha = cli.use_alpha.unwrap_or(false);
    let version = cli
        .lh_version
        .map(Version::from)
        .unwrap_or(Version::Version2);
    let (image_data, digest) = if let Some(input) = cli.input_file {
        let mut buf = String::new();
        let mut file = match File::open(input) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        };
        match file.read_to_string(&mut buf) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        };
        match from_data(buf.as_bytes(), version, module_size as usize, use_alpha) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    } else if let Some(hex) = cli.hex {
        let hex_data = match hex::decode(&hex) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        };
        match from_digest(&hex_data, version, module_size as usize, use_alpha) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    } else {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .expect("failed to read stdin");
        match from_data(
            buf.trim().as_bytes(),
            version,
            module_size as usize,
            use_alpha,
        ) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    };
    let output_file = cli.output_file.unwrap_or(PathBuf::from(format!(
        "./lifehash_{}.png",
        hex::encode(digest)
    )));
    if let Err(err) = save_image(&image_data, &output_file) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
