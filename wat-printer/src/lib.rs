use std::{error::Error, path::Path};

pub fn print(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
	let path = Path::new(input);
	
    if !path.try_exists()? {
        return Err(format!("input file not found: {}", input).into());
    }

    let wat = wasmprinter::print_file(path)?;

    std::fs::write(output, wat)?;
    
    Ok(())
}