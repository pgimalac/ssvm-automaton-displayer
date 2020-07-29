fn main() {
    cc::Build::new()
        .file("dot_to_svg.c")
        .compile("dot_to_svg");
}
