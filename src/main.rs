use std::env;
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let _ = args.next();
    let output_path = "./output/normalized.rs";

    let filename = match (args.next(), args.next()) {
        (Some(filename), None) => filename,
        _ => {
            eprintln!("Usage: representer path/to/filename.rs");
            process::exit(1);
        }
    };

    let mut input = File::open(&filename)?;
    let mut src = String::new();
    input.read_to_string(&mut src)?;

    let replaced = representer::replace(&src)?;

    let mut output = File::create(output_path)?;
    output.write(replaced.to_string().as_bytes())?;

    Ok(())
}
