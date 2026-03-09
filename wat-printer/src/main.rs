use anyhow::{Result, anyhow};
use std::path::Path;

use clap::{Arg, ArgAction, Command};

fn main() -> Result<()> {
    let matches = Command::new("Wat Printer")
        .version("1.0")
        .author("Parth Bathe <parth.bathe0142@gmail.com>")
        .about("Converts .wasm files to .wat files")
        .arg(Arg::new("input").index(1).help("The .wasm file to convert"))
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("The output .wat file, defaults to the same name as input")
                .required(false),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .short('a')
                .conflicts_with_all(["input", "output"])
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("recursive")
                .requires("all")
                .long("recursive")
                .short('r')
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("all") {
        let recursive = matches.get_flag("recursive");
        wat_printer::print_all(Path::new("."), recursive)?;
    } else {
        let Some(input) = matches.get_one::<String>("input") else {
            return Err(anyhow!("Input not provided"));
        };
        let input = Path::new(input);

        let output = matches
            .get_one::<String>("output")
            .map(|s| s.as_str())
            .map(Path::new)
            .unwrap_or(input)
            .with_extension("wat");

        wat_printer::print(input, &output)?;
    }
    Ok(())
}
