use clap::Parser;

use std::io;

mod args;
mod read_file;
mod search_replace;
mod write_file;

fn main() -> io::Result<()> {
    let args = args::Args::parse();
    if args.verbose {
        eprintln!("{:?}", current_program_name());
        eprintln!("{:#?}", args);
    }
    let lines = read_file::read(&args.src)?;
    let (exit_error, updated_lines) =
        search_replace::update(&args.start, &args.end, &args.replace, lines, args.verbose);
    write_file::write_to(&args.dst, updated_lines)?;

    if exit_error {
        std::process::exit(1); // Exit with exit code 1 (indicating an error)
    } else {
        Ok(()) // Return Ok(()) to indicate success
    }
}

fn current_program_name() -> Option<String> {
    std::env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}
