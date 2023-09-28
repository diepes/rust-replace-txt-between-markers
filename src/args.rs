use clap::Parser;

/// replacetxt : program to search for start and end markers and replace the text in between with new text.
///
/// https://github.com/diepes/rust-replace-txt-between-markers
#[derive(Parser, Debug)]
#[command(author="Pieter E Smit", version, about, long_about )]
pub struct Args {
    /// start Marker string
    #[arg(short, long)]
    pub start: String,

    /// end Marker string
    #[arg(short, long)]
    pub end: String,

    /// src filename
    #[arg(long)]
    pub src: String,

    /// dst filename
    #[arg(long)]
    pub dst: String,

    /// replacement string to place between markers
    #[arg(short, long)]
    pub replace: String,

    // verbosity
    #[clap(long, short, action)]
    pub verbose: bool,
}
