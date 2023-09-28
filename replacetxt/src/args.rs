use clap::Parser;

/// program to search for start and end marker and replace the text inbetween with new text.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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
