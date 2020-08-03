use crate::lib::regex_to_svg;
use std::io::BufRead;

mod lib;

fn main() {
    println!(
        "{}",
        regex_to_svg(
            &std::env::args()
                .nth(1)
                .or_else(|| std::io::stdin()
                    .lock()
                    .lines()
                    .next()
                    .map(Result::ok)
                    .flatten())
                .expect("A regex must be provided on stdin"),
            1,
            1
        )
    );
}
