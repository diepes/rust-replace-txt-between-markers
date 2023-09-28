// In read_file.rs

use std::fs::File;
use std::io::{self, BufRead};

pub fn read(file_name: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);

    // Create a Vec<String> to store the lines
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let file_name = "test_file.txt";

        // Write some test data to the file
        std::fs::write(file_name, "Hello\nWorld\n").expect("Failed to write test file");

        // Call the read function and check if it returns the expected lines
        let result = read(file_name).expect("Failed to read file");
        assert_eq!(result, vec!["Hello".to_string(), "World".to_string()]);

        // Clean up the temporary file (optional)
        std::fs::remove_file(file_name).expect("Failed to remove test file");
    }
}
