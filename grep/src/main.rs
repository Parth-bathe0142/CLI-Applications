use std::{error::Error, fs, process};

use grep::search;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Unable to procede due to error: {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Error: {err}");
    };
}

struct Config {
    file: String,
    query: String
}
impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            Err("Not enough arguments")
        } else {
            Ok(Self {
                file: args[1].clone(),
                query: args[2].clone()
            })
        }
    }
}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file)?;

    for line in search(&config.query, &content) {
        println!("{line}");
    }

    Ok(())
}