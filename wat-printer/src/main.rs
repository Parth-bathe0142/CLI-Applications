use std::error::Error;

use clap::{Arg, Command};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Wat Printer")
        .version("1.0")
        .author("Parth Bathe <parth.bathe0142@gmail.com>")
        .about("Converts .wasm files to .wat files")
        .arg(
            Arg::new("input")
                .index(1)
                .help("The .wasm file to convert")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("The output .wat file, defaults to the same name as input")
                .required(false),
        )
        .get_matches();

    let Some(input) = matches.get_one::<String>("input") else {
        return Err("input not provided".into());
    };

    let output = matches
        .get_one::<String>("output")
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            let path = input.trim_end_matches(".wasm");
            format!("{path}.wat")
        });

    wat_printer::print(input, &output)?;
    Ok(())
}
