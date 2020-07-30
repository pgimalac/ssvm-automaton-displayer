use crate::lib::regex_to_svg;

mod lib;

fn main() {
    println!("{:?}", regex_to_svg("a.+b*+cd?", 1, 1));
}
