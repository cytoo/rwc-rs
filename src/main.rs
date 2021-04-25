mod input;

use input::*;
use structopt::StructOpt;
use colored::Colorize;
use std::{
    path::PathBuf,
    fs::read_to_string
};

fn main() {
    let opts = Input::from_args();
    let file = opts.path;
    if !file.is_file() {
        println!(
            "\n{} {}", &file.to_str().unwrap().yellow(),
            "no such file or directory".red()
        );
        return
    }
    let content = get_file_content(&file);
    let file_name = &file.to_str().unwrap().yellow();
    println!("\n{}", file_name);

    if opts.bytes {
        println!("| {} {}", "total number of bytes:".green(),
                 content.bytes().count())
    }

    if opts.words {
        println!("| {} {}", "total number of words:".bright_green(),
                 content.split(' ').count())
    }

    if opts.chars {
        println!("| {} {}", "total number of characters:".cyan(),
                 content.chars().count())
    }

    if opts.lines {
        println!("| {} {}", "total number of lines:".blue(),
                 content.as_str().lines().count())
    }

}

fn get_file_content(file: &PathBuf) -> String {
    let cont = read_to_string(&file).unwrap();
    cont
}
