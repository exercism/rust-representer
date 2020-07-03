use clap::{App, Arg, ArgMatches};
use rust_representer::run;
use std::process;

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

fn main() {
    let matches = init_app();
    let path = format!("{}src/lib.rs", matches.value_of("path").unwrap());

    if let Err(error) = run(&path) {
        eprintln!("[error] {}", error);
        process::exit(1);
    }
}
