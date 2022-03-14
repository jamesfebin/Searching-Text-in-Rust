#![warn(clippy::pedantic)]

fn main() {
    let query = "Rust";
    let text = "\
    Rust is a multi-paradigm, 
    general-purpose programming
    language designed for performance and safety, especially safe concurrency. 
    Rust is syntactically similar to C++,
    but can guarantee memory safety 
    by using a borrow checker to 
    validate references.";

    let mut tags: Vec<usize> = vec![];
    let mut context: Vec<Vec<(usize, String)>> = vec![];
    for (i, line) in text.lines().enumerate() {
        if line.contains(query) {
            tags.push(i);

            let v = Vec::with_capacity(3);
            context.push(v);
        }
    }

    if tags.is_empty() {
        println!("Unable to find {}", query);
        return;
    }

    for (i, line) in text.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(1);
            let upper_bound = tag + 1;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_string = String::from(line);
                let context_line = (i, line_string);
                context[j].push(context_line);
            }
        }
    }

    for context_line in context.iter() {
        for &(i, ref line) in context_line.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}