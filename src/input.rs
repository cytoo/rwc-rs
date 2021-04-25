use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(about = "rwc, a tool to show you some info about a file")]
pub struct Input {

    /// path to your file.
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,

    /// show the total amount of lines.
    #[structopt(short = "l", long = "lines")]
    pub lines: bool,

    /// show the total amount of lines.
    #[structopt(short = "w", long = "words")]
    pub words: bool,

    /// show the total amount of bytes.
    #[structopt(short = "b", long = "bytes")]
    pub bytes: bool,

    /// show the total amount of lines.
    #[structopt(short = "c", long = "chars")]
    pub chars: bool,
}