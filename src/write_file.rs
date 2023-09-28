// In write_file.rs

use std::fs::File;
use std::io::{self, Write};

pub fn write_to(file_name: &str, updated_lines: Vec<String>) -> io::Result<()> {
    let mut file = File::create(file_name)?;

    for line in &updated_lines {
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?; // Add a newline after each line
    }

    Ok(())
}
