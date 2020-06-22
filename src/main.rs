use std::env;
use std::process;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let _ = args.next();

    let filename = match (args.next(), args.next()) {
        (Some(filename), None) => filename, 
        _ => {
            eprintln!("Usage: representer path/to/filename.rs");
            process::exit(1);
        }
    };

    let mut file = File::open(&filename)?;
    let mut src = String::new();
    file.read_to_string(&mut src)?;

    representer::replace(&src)?;

    Ok(())
}
