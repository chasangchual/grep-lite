use regex::Regex;

fn main() {
    let context_lines = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of papers?";
    let mut tags: Vec<usize> = Vec::new(); // list of line numbers of the matched
    let mut ctx: Vec<Vec<(bool, usize, String)>> = Vec::new(); // list of the matched/before/after lines

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i); // push matched line number

            let v = Vec::with_capacity(2*context_lines + 1); // space the space for the lines
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    /*
        haystack is a list of strings

        for (i, line) in haystack.lines().enumerate() {
        : 'line' is declared as &str

        for (i, &line) in haystack.lines().enumerate() {
        : 'line' is declared as str
     */

    for (i, line) in haystack.lines().enumerate() {
        for(j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;
            if i >= lower_bound && i <= upper_bound {
                ctx[j].push(((i==*tag), i, String::from(line))); // (i==*tag) adds matched or not status
            }
        }
    }

    for v in ctx {
        for (matched, i, line) in v {
            println!("{} {} {:?}", if matched {'m'} else {' '}, i+1, line);
        }
    }
}
