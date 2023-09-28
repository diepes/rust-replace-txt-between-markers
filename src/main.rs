use clap::Parser;

use std::io;

mod args;
mod read_file;
mod search_replace;
mod write_file;

fn main() -> io::Result<()> {
    let args = args::Args::parse();
    if args.verbose {
        eprintln!("{:?}", prog());
        eprintln!("{:#?}", args);
    }
    let lines = read_file::read(&args.src)?;
    let updated_lines =
        search_replace::update(&args.start, &args.end, &args.replace, lines, args.verbose);
    write_file::write_to(&args.dst, updated_lines)?;
    Ok(())
}

fn prog() -> Option<String> {
    std::env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}
