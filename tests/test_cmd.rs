// In your test module or a separate test file

mod tests {
    use assert_cmd::Command;
    use std::io::BufRead;
    // use std::process::Command as StdCommand;

    const FILE_SRC: &str = "test_file_src.txt";
    const FILE_DST: &str = "test_file_dst.txt";
    const START: &str = "mark-start";
    const END: &str = "mark-end";
    const REPLACE: &str = "  new multi-line\n  replacement";

    #[test]
    fn test_cmd() {
        test_create_src();
        test_main_with_parameters();
        test_check_dst();
        test_cleanup();
    }

    fn test_create_src() {
        // Write some test data to the file
        std::fs::write(
            FILE_SRC,
            "Hello\nWorld\nmark-start\nremove this\nmark-end\n",
        )
        .expect("Failed to write test file");
    }

    fn test_check_dst() {
        // Call the read function and check if it returns the expected lines
        let file = std::fs::File::open(FILE_DST).expect("Failed to read file");
        let reader = std::io::BufReader::new(file);

        // Read all lines into a Vec<String>
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

        assert_eq!(
            lines,
            vec![
                "Hello".to_string(),
                "World".to_string(),
                "mark-start".to_string(),
                "  new multi-line".to_string(),
                "  replacement".to_string(),
                "mark-end".to_string()
            ]
        );
    }

    fn test_cleanup() {
        // Clean up the temporary file (optional)
        std::fs::remove_file(FILE_SRC).expect("Failed to remove test file");
        std::fs::remove_file(FILE_DST).expect("Failed to remove test file");
    }

    fn test_main_with_parameters() {
        // Create a Command instance with your binary (replace "your_binary" with your binary's name)
        let mut cmd = Command::cargo_bin("replacetxt").unwrap();

        // Add the command-line arguments
        cmd.arg("--src")
            .arg(FILE_SRC)
            .arg("--dst")
            .arg(FILE_DST)
            .arg("--start")
            .arg(START)
            .arg("--end")
            .arg(END)
            .arg("--replace")
            .arg(REPLACE)
            .arg("--verbose");

        // Run the command and capture the output
        let assert = cmd.assert();

        // Assert that the command executed successfully
        assert.success();

        // You can also capture and inspect the output if needed
        //let output = assert.get_output();
        // You can check the output.stdout and output.stderr fields for verification
        // For example, you can assert that the output contains specific strings or patterns.
    }
}
