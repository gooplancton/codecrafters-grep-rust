use std::env;
use std::io;
use std::process;

mod matchers;
mod utils;

use matchers::alternation::Alternation;
use matchers::Match;
use matchers::Matcher;

fn regex_match(pattern: &str, line: &str) -> Option<(usize, Match)> {
    for offset in 0..line.len() {
        let matcher = Alternation::new(pattern, "", None);
        let this_match = Match::new(offset);
        match matcher.extend_from(line, this_match) {
            Ok(this_match) => return Some((offset, this_match)),
            Err(_) => continue,
        }
    }

    None
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let pattern = pattern.as_str();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();
    let match_result = regex_match(pattern, &input_line);

    if let Some((offset, this_match)) = match_result {
        println!(
            "Matched substring: {}",
            input_line.get(offset..this_match.offset).unwrap()
        );

        this_match
            .captures
            .into_iter()
            .enumerate()
            .for_each(|(idx, capture)| {
                let captured_string = input_line.get(capture.start..capture.end.unwrap()).unwrap();
                println!("Capture #{}: {}", idx + 1, captured_string);
            });

        process::exit(0)
    } else {
        println!("Did not match");

        process::exit(1)
    }
}
