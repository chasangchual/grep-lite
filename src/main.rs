use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{Arg, arg, Command, ArgAction};

fn main() {
    // Command which is a part of clap crate (https://docs.rs/clap/latest/clap/) handles the command line arguments.
    // with .arg(...), the argument(s) are defined. The defined arguments can be explained with --h option
    /*
        grep lite in Rus

        Usage: grep-lite [OPTIONS] --pattern <regexpr>

        Options:
          -p, --pattern <regexpr>  the regular expression pattern to search for
          -f, --file <file>        Read one or more newline separated patterns from file.
          -v, --verbose
          -h, --help               Print help information
          -V, --version            Print version information
     */
    let matches = Command::new("grep-lite")
        .version("0.1")
        .author("Sangchual CHA <sangchual.cha@gmail.com>")
        .about("grep lite in Rus")
        .arg(arg!(--pattern <regexpr>)
            .short('p')
            .help("the regular expression pattern to search for")
            .required(true))
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .required(false)
            .help("Read one or more newline separated patterns from file."))
        .arg(Arg::new("verbose").short('v').long("verbose").required(false).action(ArgAction::SetTrue))
        .get_matches();

    let _search_pattern = matches.get_one::<String>("pattern").unwrap();
    let _file_to_read = matches.get_one::<String>("file").unwrap_or(&String::from(""));
    let _quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of papers?";

    let re = Regex::new(_search_pattern.as_str()).unwrap(); // unwrap(), if any error condition occurres while running new(), it will terminate

    for (_i, line) in _quote.lines().enumerate() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(matched) => println!("{} \n - {:?}", line, matched), // 'matched' prints Match { text: "dark square is a picture feverishly turned--in search of what?", start: 17, end: 24 }
            None => (),
        }
    }
}
