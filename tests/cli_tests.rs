use lifehash_lib::{run, Either, Version};
use std::path::PathBuf;

#[test]
fn test_parse_cli_from_hex() {
    let args = vec![
        "lifehash",
        "--hex",
        "010203",
        "--module-size",
        "1",
        "--use-alpha",
        "false",
        "--lh-version",
        "2",
        "--out-file",
        "output.png",
    ];
    let runtime_args = lifehash_lib::parse_cli_from(args).expect("Failed to parse CLI args");
    assert_eq!(runtime_args.module_size, 1);
    assert_eq!(runtime_args.use_alpha, false);
    assert_eq!(runtime_args.version, Version::Version2);
    assert_eq!(runtime_args.output_file, PathBuf::from("output.png"));
    // Since a hex argument was provided, we expect `parse_input` to decode it.
    match runtime_args.input {
        Either::Right(ref bytes) => {
            assert_eq!(bytes, &vec![1, 2, 3]);
        }
        Either::Left(_) => {
            panic!("Expected hex input to produce bytes");
        }
    }
}

#[test]
fn test_parse_cli_from_invalid_hex() {
    let args = vec![
        "lifehash",
        "--hex",
        "someinvalidhex",
        "--module-size",
        "1",
        "--use-alpha",
        "false",
        "--lh-version",
        "2",
        "--out-file",
        "output.png",
    ];
    assert!(lifehash_lib::parse_cli_from(args).is_err());
}

#[test]
fn test_parse_cli_from_invalid_module_size() {
    let args = vec![
        "lifehash",
        "--hex",
        "010203",
        "--module-size",
        "0",
        "--use-alpha",
        "false",
        "--lh-version",
        "2",
    ];
    let runtime_args = lifehash_lib::parse_cli_from(args).expect("Failed to parse CLI args");
    assert!(run(runtime_args).is_err());
}

#[test]
fn test_parse_cli_vcersion_1() {
    let args = vec![
        "lifehash",
        "--hex",
        "010203",
        "--module-size",
        "1",
        "--use-alpha",
        "false",
        "--lh-version",
        "1",
        "--out-file",
        "output.png",
    ];
    let runtime_args = lifehash_lib::parse_cli_from(args).expect("Failed to parse CLI args");
    assert_eq!(runtime_args.module_size, 1);
    assert_eq!(runtime_args.use_alpha, false);
    assert_eq!(runtime_args.version, Version::Version1);
    assert_eq!(runtime_args.output_file, PathBuf::from("output.png"));
    // Since a hex argument was provided, we expect `parse_input` to decode it.
    match runtime_args.input {
        Either::Right(ref bytes) => {
            assert_eq!(bytes, &vec![1, 2, 3]);
        }
        Either::Left(_) => {
            panic!("Expected hex input to produce bytes");
        }
    }
}
#[test]
fn test_parse_cli_vcersion_3() {
    let args = vec![
        "lifehash",
        "--hex",
        "010203",
        "--module-size",
        "1",
        "--use-alpha",
        "false",
        "--lh-version",
        "3",
        "--out-file",
        "output.png",
    ];
    let runtime_args = lifehash_lib::parse_cli_from(args).expect("Failed to parse CLI args");
    assert_eq!(runtime_args.module_size, 1);
    assert_eq!(runtime_args.use_alpha, false);
    assert_eq!(runtime_args.version, Version::Detailed);
    assert_eq!(runtime_args.output_file, PathBuf::from("output.png"));
    // Since a hex argument was provided, we expect `parse_input` to decode it.
    match runtime_args.input {
        Either::Right(ref bytes) => {
            assert_eq!(bytes, &vec![1, 2, 3]);
        }
        Either::Left(_) => {
            panic!("Expected hex input to produce bytes");
        }
    }
}
#[test]
fn test_parse_cli_vcersion_4() {
    let args = vec![
        "lifehash",
        "--hex",
        "010203",
        "--module-size",
        "1",
        "--use-alpha",
        "false",
        "--lh-version",
        "4",
        "--out-file",
        "output.png",
    ];
    let runtime_args = lifehash_lib::parse_cli_from(args).expect("Failed to parse CLI args");
    assert_eq!(runtime_args.module_size, 1);
    assert_eq!(runtime_args.use_alpha, false);
    assert_eq!(runtime_args.version, Version::Fiducial);
    assert_eq!(runtime_args.output_file, PathBuf::from("output.png"));
    // Since a hex argument was provided, we expect `parse_input` to decode it.
    match runtime_args.input {
        Either::Right(ref bytes) => {
            assert_eq!(bytes, &vec![1, 2, 3]);
        }
        Either::Left(_) => {
            panic!("Expected hex input to produce bytes");
        }
    }
}
#[test]
fn test_parse_cli_vcersion_5() {
    let args = vec![
        "lifehash",
        "--hex",
        "010203",
        "--module-size",
        "1",
        "--use-alpha",
        "false",
        "--lh-version",
        "5",
        "--out-file",
        "output.png",
    ];
    let runtime_args = lifehash_lib::parse_cli_from(args).expect("Failed to parse CLI args");
    assert_eq!(runtime_args.module_size, 1);
    assert_eq!(runtime_args.use_alpha, false);
    assert_eq!(runtime_args.version, Version::GrayscaleFiducial);
    assert_eq!(runtime_args.output_file, PathBuf::from("output.png"));
    // Since a hex argument was provided, we expect `parse_input` to decode it.
    match runtime_args.input {
        Either::Right(ref bytes) => {
            assert_eq!(bytes, &vec![1, 2, 3]);
        }
        Either::Left(_) => {
            panic!("Expected hex input to produce bytes");
        }
    }
}

#[test]
fn test_parse_cli_from_file() {
    use std::io::Write;
    use tempfile::NamedTempFile;
    let mut temp_file = NamedTempFile::new().expect("failed to create temp file");
    let file_content = "Hello, world!";
    write!(temp_file, "{}", file_content).expect("failed to write to temp file");
    let args = vec![
        "lifehash",
        "--in-file",
        temp_file
            .path()
            .as_os_str()
            .to_str()
            .expect("failed to convert to str"),
        "--module-size",
        "1",
        "--use-alpha",
        "false",
        "--lh-version",
        "2",
        "--out-file",
        "output.png",
    ];
    let runtime_args = lifehash_lib::parse_cli_from(args).expect("Failed to parse CLI args");
    assert_eq!(runtime_args.module_size, 1);
    assert_eq!(runtime_args.use_alpha, false);
    assert_eq!(runtime_args.version, Version::Version2);
    assert_eq!(runtime_args.output_file, PathBuf::from("output.png"));
    // Since a hex argument was provided, we expect `parse_input` to decode it.
    match runtime_args.input {
        Either::Right(_) => {
            panic!("Expected file input to produce string");
        }
        Either::Left(string) => {
            assert_eq!(string, file_content.to_string());
        }
    }
}
