use clap::{App, Arg, ArgMatches};
use rust_representer::replace;
use std::fs::File;
use std::io::{prelude::*, Read};

const OUTPUT: &'static str = "representation.rs";

fn init_app<'a>() -> ArgMatches<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("slug")
                .short("s")
                .long("slug")
                .help("The slug of the exercise to be analyzed (e.g. 'reverse-string').")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .help("A path to a directory containing the submitted file(s).")
                .takes_value(true)
                .required(true),
        )
        .get_matches()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = init_app();

    let path = matches.value_of("path").unwrap();
    let filename = format!("{}src/lib.rs", path);

    let mut input = File::open(&filename)?;
    let mut src = String::new();
    input.read_to_string(&mut src)?;

    let replaced = replace(&src)?;

    let mut output = File::create(format!("{}{}", path, OUTPUT))?;
    output.write(replaced.to_string().as_bytes())?;

    Ok(())
}
