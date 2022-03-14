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

// fn main() {
//     let ctx_lines = 2;
//     let needle = "oo";
//     let text ="abc";
//     let haystack = "\
//   Every face, every shop,
//   bedroom window, public-house, and
//   dark square is a picture
//   feverishly turned--in search of what?
//   It is the same with books.
//   What do we seek
//   through millions of pages?";

//     let mut tags: Vec<usize> = vec![];               // <1>
//     let mut ctx: Vec<Vec<(
//                  usize, String)>> = vec![];          // <2>
//     for (i, line) in haystack.lines().enumerate() {  // <3>
//       if line.contains(needle) {
//         tags.push(i);
//         let v = Vec::with_capacity(2*ctx_lines + 1); // <4>
//         ctx.push(v);
//       }
//     }
//     if tags.is_empty() {                             // <5>
//       return;
//     }
//     for (i, line) in haystack.lines().enumerate() {  // <6>
//       for (j, tag) in tags.iter().enumerate() {
//         let lower_bound =
//             tag.saturating_sub(ctx_lines);           // <7>
//         let upper_bound =
//             tag + ctx_lines;
//         if (i >= lower_bound) && (i <= upper_bound) {
//             let line_as_string = String::from(line); // <8>
//             let local_ctx = (i, line_as_string);
//             ctx[j].push(local_ctx);
//         }
//       }
//     }
//     for local_ctx in ctx.iter() {
//       for &(i, ref line) in local_ctx.iter() {       // <9>
//         let line_num = i + 1;
//         println!("{}: {}", line_num, line);
//       }
//     }
//   }
