use regex::Regex;
use clap::{Arg, arg, Command, ArgAction};

fn main() {
    let matches = Command::new("grep-lite")
        .version("0.1")
        .author("Sangchual CHA <sangchual.cha@gmail.com>")
        .about("grep lite version in Rus")
        .arg(arg!(--pattern <regexpr>)
            .short('p')
            .help("the regular expression pattern to search for")
            .required(true))
        .arg(Arg::new("verbose").short('v').long("verbose").required(false).action(ArgAction::SetTrue))
        .get_matches();

    let search_pattern = matches.get_one::<String>("pattern").unwrap();
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of papers?";

    let re = Regex::new(search_pattern.as_str()).unwrap(); // unwrap(), if any error condition occurres while running new(), it will terminate


    for (_i, line) in quote.lines().enumerate() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(matched) => println!("{} \n - {:?}", line, matched), // 'matched' prints Match { text: "dark square is a picture feverishly turned--in search of what?", start: 17, end: 24 }
            None => (),
        }
    }
}
