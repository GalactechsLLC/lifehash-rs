use lifehash_lib::parse_cli;
use lifehash_lib::run;
use std::io::Error;

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), Error> {
    run(parse_cli()?)
}
