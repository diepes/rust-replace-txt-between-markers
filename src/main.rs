use clap::Parser;

use std::io;

mod args;
mod read_file;
mod search_replace;
mod write_file;

fn main() -> io::Result<()> {
    let args = args::Args::parse();
    println!("{:#?}", args);
    // for _ in 0..len(args) {
    //     println!("Hello {}!", args.name)
    // }
    // Call the read function from read_file module
    let lines = read_file::read(&args.src)?;
    let updated_lines =
        search_replace::update(&args.start, &args.end, &args.replace, lines, args.verbose);
    write_file::write_to(&args.dst, updated_lines)?;
    Ok(())
}
