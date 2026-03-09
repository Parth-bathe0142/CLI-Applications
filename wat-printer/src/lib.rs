use anyhow::{Error, Result};
use std::{fs, path::Path};

pub fn print(input: &Path, output: &Path) -> Result<()> {
    if !input.try_exists()? {
        return Err(Error::msg("Could not find the specified file"));
    }

    let wat = wasmprinter::print_file(input)?;

    std::fs::write(output, wat)?;

    Ok(())
}

fn print_if_wasm(path: &Path) -> Result<()> {
    if path.extension().unwrap_or_default() != "wasm" {
        return Ok(());
    }

    let wat = wasmprinter::print_file(path)?;
    let output = path.with_extension("wat");
    std::fs::write(&output, wat)?;
    Ok(())
}

pub fn print_all(path: &Path, recursive: bool) -> Result<()> {
    if recursive {
        visit_dirs(path, &print_if_wasm)?;
    } else {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    print_if_wasm(&path)?;
                }
            }
        } else {
            print_if_wasm(path)?;
        }
    }
    Ok(())
}

fn visit_dirs(path: &Path, cb: &impl Fn(&Path) -> Result<()>) -> Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&path)?;
            }
        }
    }
    Ok(())
}
