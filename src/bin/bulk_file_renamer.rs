use std::{fs, io, path::{Path, PathBuf}};

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Bulk File Renamer")
        .version("1.0")
        .author("Parth Bathe <parth.bathe0142@gmail.com>")
        .about("Renames all files in a given folder by replacing parts of file names")

        .arg(Arg::new("path")
             .short('p')
             .long("path")
             .required(true)
             .help("Path of the folder whose files are to be renamed"))

        .arg(Arg::new("replace")
             .short('r')
             .long("replace")
             .aliases(&["remove"])
             .conflicts_with_all(&["start", "end"])
             .help("The text present in the file names that is to be replaced"))

        .arg(Arg::new("start")
             .short('s')
             .long("start")
             .aliases(&["preppend", "prefix"])
             .conflicts_with_all(&["replace", "end"])
             .requires("with")
             .num_args(0)
             .help("Specifies that the addition is to be made to the start of the file names"))
             
        .arg(Arg::new("end")
             .short('e')
             .long("end")
             .aliases(&["append", "suffix"])
             .conflicts_with_all(&["replace", "start"])
             .requires("with")
             .num_args(0)
             .help("Specifies that the addition is to be made to the end of the file names"))

        .arg(Arg::new("with")
             .short('w')
             .long("with")
             .help("The replacement or addition to be made to the file names"))

        .get_matches();

    if let Some(path) = matches.get_one::<String>("path") {
        let path = Path::new(path);
        let error_msg = "Error: Invalid path, make sure it points to a valid, accessible directory".to_string();
        
        if !path.try_exists().unwrap_or(false) || !path.is_dir() {
            eprintln!("{}", error_msg);
            return;
        }

        let modification = if let Some(replace) = matches.get_one::<String>("replace") {
            Modification::Replacement(replace.clone())
        } else if matches.contains_id("start") {
            Modification::Start
        } else if matches.contains_id("end") {
            Modification::End
        } else {
            panic!("No replacement or position provided")
        };

        let with = matches.get_one::<String>("with").unwrap_or(&String::new()).clone();

        visit_dirs(path, &mut |entry: &PathBuf| {

            let old = entry.file_name().unwrap().to_string_lossy().to_string();

            let new = match &modification {
                Modification::Replacement(r) => {
                    old.replace(r, &with)
                },
                Modification::Start => {
                    format!("{}{}", with, old)
                },
                Modification::End => {
                    format!("{}{}", old, with)
                    
                },
            };

            let old_path = entry.as_path();
            let new_path = old_path.with_file_name(new);
            match fs::rename(old_path, &new_path) {
                Ok(_) => println!("Renamed: {:?} -> {:?}", old_path, new_path),
                Err(err) => eprintln!("\nFailed to rename {:?} -> {:?}: {}\n", old_path, new_path, err),
            }
        }).expect("Error while Renaming files");
         
    }
}

enum Modification {
    Replacement(String),
    Start,
    End
}

fn visit_dirs(path: &Path, cb: &mut dyn FnMut(&PathBuf) -> ()) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&path);
            }
        }
    }
    Ok(())
}