#[test]
pub fn test_hello_version1() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let (image, _) = from_data(b"Hello", Version1, 1, false).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        0, 27, 7, 0, 24, 6, 0, 33, 9, 0, 29, 8, 0, 29, 8, 0, 28, 8, 0, 32, 9, 0, 87, 24, 0, 87, 24,
        0, 89, 25,
    ];
    assert_eq!(image.pixels[0..30], expected);
}

#[test]
pub fn test_hello_version2() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let (image, _) = from_data(b"Hello", Version2, 1, false).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        146, 126, 130, 178, 104, 92, 182, 101, 87, 202, 88, 64, 199, 89, 66, 197, 90, 69, 182, 101,
        87, 180, 102, 89, 159, 117, 114, 210, 82, 54,
    ];
    assert_eq!(image.pixels[0..30], expected);
}

#[test]
pub fn test_hello_detailed() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let (image, _) = from_data(b"Hello", Detailed, 1, false).unwrap();
    assert_eq!(image.width, 64);
    assert_eq!(image.height, 64);
    let expected = [
        52, 94, 41, 45, 81, 35, 45, 82, 36, 45, 82, 36, 45, 82, 36, 36, 65, 28, 36, 65, 28, 36, 64,
        28, 59, 107, 47, 59, 107, 47,
    ];
    assert_eq!(image.pixels[0..30], expected);
}

#[test]
pub fn test_hello_fiduciary() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let (image, _) = from_data(b"Hello", Fiducial, 1, false).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        47, 20, 29, 5, 2, 3, 0, 0, 0, 2, 0, 1, 0, 0, 0, 66, 29, 41, 68, 29, 42, 69, 30, 43, 97, 42,
        60, 96, 42, 59,
    ];
    assert_eq!(image.pixels[0..30], expected);
}

#[test]
pub fn test_hello_grayscale_fiduciary() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let (image, _) = from_data(b"Hello", GrayscaleFiducial, 1, false).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        48, 48, 48, 47, 47, 47, 63, 63, 63, 64, 64, 64, 23, 23, 23, 22, 22, 22, 23, 23, 23, 62, 62,
        62, 64, 64, 64, 64, 64, 64,
    ];
    assert_eq!(image.pixels[0..30], expected);
}

#[test]
pub fn test_hello_version1_alpha() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let (image, _) = from_data(b"Hello", Version1, 1, true).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        0, 27, 7, 255, 0, 24, 6, 255, 0, 33, 9, 255, 0, 29, 8, 255, 0, 29, 8, 255, 0, 28, 8, 255,
        0, 32, 9, 255, 0, 87, 24, 255, 0, 87, 24, 255, 0, 89, 25, 255,
    ];
    assert_eq!(image.pixels[0..40], expected);
}

#[test]
pub fn test_hello_version2_alpha() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
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

#[test]
pub fn test_hello_detailed_alpha() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let (image, _) = from_data(b"Hello", Detailed, 1, true).unwrap();
    assert_eq!(image.width, 64);
    assert_eq!(image.height, 64);
    let expected = [
        52, 94, 41, 255, 45, 81, 35, 255, 45, 82, 36, 255, 45, 82, 36, 255, 45, 82, 36, 255, 36,
        65, 28, 255, 36, 65, 28, 255, 36, 64, 28, 255, 59, 107, 47, 255, 59, 107, 47, 255,
    ];
    assert_eq!(image.pixels[0..40], expected);
}

#[test]
pub fn test_hello_fiduciary_alpha() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let (image, _) = from_data(b"Hello", Fiducial, 1, true).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        47, 20, 29, 255, 5, 2, 3, 255, 0, 0, 0, 255, 2, 0, 1, 255, 0, 0, 0, 255, 66, 29, 41, 255,
        68, 29, 42, 255, 69, 30, 43, 255, 97, 42, 60, 255, 96, 42, 59, 255,
    ];
    assert_eq!(image.pixels[0..40], expected);
}

#[test]
pub fn test_hello_grayscale_fiduciary_alpha() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let (image, _) = from_data(b"Hello", GrayscaleFiducial, 1, true).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        48, 48, 48, 255, 47, 47, 47, 255, 63, 63, 63, 255, 64, 64, 64, 255, 23, 23, 23, 255, 22,
        22, 22, 255, 23, 23, 23, 255, 62, 62, 62, 255, 64, 64, 64, 255, 64, 64, 64, 255,
    ];
    assert_eq!(image.pixels[0..40], expected);
}

#[test]
pub fn test_select_gradient_complementary_version_1() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let data = "Hello Hello Hello Hello Hello Hello Hello".to_string(); //This triggers complementary in select_gradient
    let (image, _) = from_data(data.as_bytes(), Version1, 1, false).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        192, 27, 128, 196, 22, 129, 195, 23, 129, 195, 23, 129, 193, 26, 128, 195, 23, 129, 179,
        43, 126, 179, 43, 126, 193, 25, 129, 196, 22, 129, 194, 24, 129, 194, 24, 129, 196, 22,
        129, 196,
    ];
    assert_eq!(image.pixels[0..40], expected);
}

#[test]
pub fn test_select_gradient_triadic_version_1() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let data = "Hello Hello".to_string(); //This triggers triadic in select_gradient
    let (image, _) = from_data(data.as_bytes(), Version1, 1, false).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        80, 54, 108, 88, 53, 106, 163, 45, 86, 193, 42, 79, 178, 44, 83, 178, 44, 83, 133, 48, 94,
        200, 41, 77, 223, 52, 73, 232, 41, 69, 219, 58, 75, 215, 40, 73, 215, 40, 73, 208,
    ];
    assert_eq!(image.pixels[0..40], expected);
}

#[test]
pub fn test_select_gradient_analogous_version_1() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let data = "Hello Hello Hello Hello Hello Hello".to_string(); //This triggers analogous in select_gradient
    let (image, _) = from_data(data.as_bytes(), Version1, 1, false).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        185, 70, 67, 253, 236, 188, 253, 236, 188, 252, 233, 185, 253, 235, 187, 252, 233, 185,
        252, 228, 181, 252, 230, 182, 251, 222, 175, 250, 204, 159, 249, 200, 157, 249, 199, 155,
        134, 22, 39, 123,
    ];
    assert_eq!(image.pixels[0..40], expected);
}

#[test]
pub fn test_select_gradient_triadic_version_2() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let data = "Hello Hello Hello Hello".to_string(); //This triggers triadic in select_gradient
    let (image, _) = from_data(data.as_bytes(), Version2, 1, false).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        180, 61, 63, 76, 51, 92, 64, 50, 95, 58, 50, 97, 46, 49, 100, 58, 50, 97, 119, 55, 80, 119,
        55, 80, 119, 55, 80, 119, 55, 80, 119, 55, 80, 119, 55, 80, 101, 54, 85, 76,
    ];
    assert_eq!(image.pixels[0..40], expected);
}

#[test]
pub fn test_select_gradient_analogous_version_2() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let data = "Hello Hello".to_string(); //This triggers analogous in select_gradient
    let (image, _) = from_data(data.as_bytes(), Version2, 1, false).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        81, 42, 93, 53, 37, 81, 89, 44, 96, 220, 109, 160, 220, 109, 160, 220, 109, 160, 215, 103,
        157, 215, 103, 157, 215, 103, 157, 220, 106, 160, 208, 100, 154, 188, 90, 144, 181, 87,
        141, 21,
    ];
    assert_eq!(image.pixels[0..40], expected);
}

#[test]
pub fn test_select_gradient_complementary_fiducial() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let data = "Hello Hello Hello Hello Hello Hello Hello".to_string(); //This triggers complementary_fiducial in select_gradient
    let (image, _) = from_data(data.as_bytes(), Fiducial, 1, false).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        0, 81, 29, 10, 90, 40, 0, 67, 12, 0, 77, 24, 0, 69, 14, 0, 71, 17, 0, 71, 17, 0, 60, 4, 0,
        56, 0, 0, 56, 0, 0, 77, 24, 0, 74, 20, 0, 30, 0, 0,
    ];
    assert_eq!(image.pixels[0..40], expected);
}

#[test]
pub fn test_select_gradient_triadic_fiducial() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let data = "Hello Hello".to_string(); //This triggers triadic_fiducial in select_gradient
    let (image, _) = from_data(data.as_bytes(), Fiducial, 1, false).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        152, 173, 197, 25, 72, 127, 25, 72, 127, 149, 171, 196, 28, 74, 128, 25, 72, 127, 162, 181,
        203, 160, 179, 202, 165, 183, 204, 167, 185, 206, 165, 183, 204, 185, 104, 109, 185, 104,
        109, 188,
    ];
    assert_eq!(image.pixels[0..40], expected);
}

#[test]
pub fn test_select_gradient_analogous_fiducial() {
    use lifehash_lib::lifehash::from_data;
    use lifehash_lib::Version::*;
    let data = "Hello Hello Hello Hello Hello Hello".to_string(); //This triggers analogous_fiducial in select_gradient
    let (image, _) = from_data(data.as_bytes(), Fiducial, 1, false).unwrap();
    assert_eq!(image.width, 32);
    assert_eq!(image.height, 32);
    let expected = [
        105, 46, 0, 98, 37, 0, 103, 43, 0, 103, 43, 0, 103, 43, 0, 103, 43, 0, 103, 43, 0, 103, 43,
        0, 98, 37, 0, 124, 73, 25, 124, 73, 25, 139, 94, 51, 141, 96, 54, 141,
    ];
    assert_eq!(image.pixels[0..40], expected);
}
