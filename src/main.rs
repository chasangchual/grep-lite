fn main() {
    let search_term = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of papers?";
    let mut line_num = 0;

    for line in quote.lines() {
        line_num += 1;
        if line.contains(search_term) {
            println!("{} {}", line_num, line);
        }
    }
}
