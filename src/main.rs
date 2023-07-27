use clap::{Arg, ArgMatches, Command};
use rust_representer::run;
use std::process;

fn init_app() -> ArgMatches {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("slug")
                .short('s')
                .long("slug")
                .help("The slug of the exercise to be analyzed (e.g. 'reverse-string').")
                .required(true),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("A path to a directory containing the submitted file(s).")
                .required(true),
        )
        .get_matches()
}

fn main() {
    let matches = init_app();
    let path = format!("{}src/lib.rs", matches.get_one::<String>("path").unwrap());

    if let Err(error) = run(&path) {
        eprintln!("[error] {}", error);
        process::exit(1);
    }
}
