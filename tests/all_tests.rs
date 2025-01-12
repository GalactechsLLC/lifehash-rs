#[test]
pub fn test_hello() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::Version2;
    use std::path::PathBuf;
    let (image, digest) = from_data(b"Hello", Version2, 1, false).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        146, 126, 130, 178, 104, 92, 182, 101, 87, 202, 88, 64, 199, 89, 66, 197, 90, 69, 182, 101,
        87, 180, 102, 89, 159, 117, 114, 210, 82, 54,
    ];
    assert_eq!(image.pixels[0..30], expected);
    let output_file = PathBuf::from(format!("./lifehash_{}.png", hex::encode(digest)));
    if let Err(err) = lifehash_lib::save_image(&image, &output_file) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

#[test]
pub fn test_hello_alpha() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::Version2;
    let (image, _) = from_data(b"Hello", Version2, 1, true).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        146, 126, 130, 255, 178, 104, 92, 255, 182, 101, 87, 255, 202, 88, 64, 255, 199, 89, 66,
        255, 197, 90, 69, 255, 182, 101, 87, 255, 180, 102, 89, 255, 159, 117, 114, 255, 210, 82,
        54, 255,
    ];
    assert_eq!(image.pixels[0..40], expected);
}
