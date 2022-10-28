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
    let mut tags: Vec<usize> = Vec::new();
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            let v = Vec::with_capacity(2*context_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for(j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;
            if i >= lower_bound && i <= upper_bound {
                ctx[j].push((i, String::from(line)));
            }
        }
    }

    for v in ctx {
        for (i, line) in v {
            println!("{} {}", i+1, line);
        }
    }
}
