use regex::Regex;

fn main() {
    let search_term = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of papers?";

    let re = Regex::new(search_term).unwrap(); // unwrap(), if any error condition occurres while running new(), it will terminate


    for (_i, line) in quote.lines().enumerate() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(matched) => println!("{} \n - {:?}", line, matched), // 'matched' prints Match { text: "dark square is a picture feverishly turned--in search of what?", start: 17, end: 24 }
            None => (),
        }
    }
}
