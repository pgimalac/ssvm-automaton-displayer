fn main() {
    println!("cargo:rustc-link-lib=gvc");
    println!("cargo:rustc-link-lib=cgraph");

    cc::Build::new()
        .file("dot_to_svg.c")
        .compile("dot_to_svg");
}
