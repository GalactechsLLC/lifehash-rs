use lifehash_lib::utils::modulo;

#[test]
fn test_parse_zero_saturation() {
    let rgb = lifehash_lib::colors::hsb::Color::new(1.0, 0.0, 1.0)
        .rgb()
        .unwrap();
    let expected = lifehash_lib::colors::rgb::Color::new(1.0, 1.0, 1.0);
    assert_eq!(rgb, expected);
}
#[test]
fn test_parse_zero_hue() {
    let rgb = lifehash_lib::colors::hsb::Color::new(0.0, 1.0, 1.0)
        .rgb()
        .unwrap();
    let expected = lifehash_lib::colors::rgb::Color::new(1.0, 0.0, 0.0);
    assert_eq!(rgb, expected);
}
#[test]
fn test_parse_zero_brightness() {
    let rgb = lifehash_lib::colors::hsb::Color::new(1.0, 1.0, 0.0)
        .rgb()
        .unwrap();
    let expected = lifehash_lib::colors::rgb::Color::new_u8(0, 0, 0);
    assert_eq!(rgb, expected);
}

#[test]
fn test_parse_90_hue() {
    let rgb = lifehash_lib::colors::hsb::Color::new(0.9, 1.0, 1.0)
        .rgb()
        .unwrap();
    let mut calc_hue = modulo(0.9, 1.0);
    calc_hue *= 6.0;
    let expected = lifehash_lib::colors::rgb::Color::new(
        1.0,
        0.0,
        1.0 - 1.0 * (calc_hue - (calc_hue.floor() as i32) as f64),
    );
    assert_eq!(rgb, expected);
}

#[test]
fn test_parse_75_hue() {
    let rgb = lifehash_lib::colors::hsb::Color::new(0.75, 1.0, 1.0)
        .rgb()
        .unwrap();
    let mut calc_hue = modulo(0.75, 1.0);
    calc_hue *= 6.0;
    let expected = lifehash_lib::colors::rgb::Color::new(
        1.0 - 1.0 * (calc_hue - (calc_hue.floor() as i32) as f64),
        0.0,
        1.0,
    );
    assert_eq!(rgb, expected);
}

#[test]
fn test_parse_60_hue() {
    let rgb = lifehash_lib::colors::hsb::Color::new(0.6, 1.0, 1.0)
        .rgb()
        .unwrap();
    let mut calc_hue = modulo(0.6, 1.0);
    calc_hue *= 6.0;
    let expected = lifehash_lib::colors::rgb::Color::new(
        0.0,
        1.0 - 1.0 * (calc_hue - (calc_hue.floor() as i32) as f64),
        1.0,
    );
    assert_eq!(rgb, expected);
}

#[test]
fn test_parse_30_hue() {
    let rgb = lifehash_lib::colors::hsb::Color::new(0.3, 1.0, 1.0)
        .rgb()
        .unwrap();
    let mut calc_hue = modulo(0.3, 1.0);
    calc_hue *= 6.0;
    let expected = lifehash_lib::colors::rgb::Color::new(
        1.0 - 1.0 * (calc_hue - (calc_hue.floor() as i32) as f64),
        1.0,
        0.0,
    );
    assert_eq!(rgb, expected);
}

#[test]
fn test_parse_20_hue() {
    let rgb = lifehash_lib::colors::hsb::Color::new(0.2, 1.0, 1.0)
        .rgb()
        .unwrap();
    let mut calc_hue = modulo(0.2, 1.0);
    calc_hue *= 6.0;
    let expected = lifehash_lib::colors::rgb::Color::new(
        1.0 - 1.0 * (calc_hue - (calc_hue.floor() as i32) as f64),
        1.0,
        0.0,
    );
    assert_eq!(rgb, expected);
}

#[test]
fn test_parse_10_hue() {
    let rgb = lifehash_lib::colors::hsb::Color::new(0.1, 1.0, 1.0)
        .rgb()
        .unwrap();
    let mut calc_hue = modulo(0.1, 1.0);
    calc_hue *= 6.0;
    let expected = lifehash_lib::colors::rgb::Color::new(
        1.0,
        1.0 - 1.0 * (1.0 - (calc_hue - (calc_hue.floor() as i32) as f64)),
        0.0,
    );
    assert_eq!(rgb, expected);
}
