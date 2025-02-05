[![codecov](https://codecov.io/gh/GalactechsLLC/lifehash-rs/graph/badge.svg?token=JJK4MAWNBY)](https://codecov.io/gh/GalactechsLLC/lifehash-rs)

# lifehash - The Rust implementation

LifeHash is a method of hash visualization. This is a rust implementation of [bc-lifehash](https://github.com/BlockchainCommons/bc-lifehash), see there for an overview of LifeHash.

## Dependencies

- [clap](https://github.com/clap-rs/clap) for Command Line Parsing 
- [hex](https://github.com/KokaKiwi/rust-hex) for encoding and decoding hex
- [png](https://github.com/image-rs/image-png) for saving created image to file in png format
- [sha2](https://github.com/RustCrypto/hashes) for hashing input data

## Installation
```bash
$ cargo install lifehash
```

## Using the CLI

```bash
$ echo "Hello" | lifehash -m 4
$ lifehash -m 4 -x 185f8db32271fe25f561a6fc938b2e264306ec304eda518007d1764826381969
```

## Testing

```bash
$ cargo test --package lifehash
```
