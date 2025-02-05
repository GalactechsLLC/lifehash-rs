#[test]
pub fn test_save_image() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    use std::path::PathBuf;
    let (image, digest) = from_data(b"Hello", Version2, 1, false).unwrap();
    let output_file = PathBuf::from(format!("./lifehash_{}.png", hex::encode(digest)));
    if let Err(err) = lifehash_lib::save_image(&image, &output_file) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

#[test]
pub fn test_save_image_alpha() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    use std::path::PathBuf;
    let (image, digest) = from_data(b"Hello", Version2, 1, false).unwrap();
    let output_file = PathBuf::from(format!("./lifehash_{}.png", hex::encode(digest)));
    if let Err(err) = lifehash_lib::save_image(&image, &output_file) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
