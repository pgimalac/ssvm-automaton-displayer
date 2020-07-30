extern crate cc;

fn main() {
    // println!("cargo:rustc-link-lib=mygvc");
    // println!("cargo:rustc-link-lib=gvc");
    // println!("cargo:rustc-link-lib=cgraph");

    cc::Build::new()
        .file("src/c/mygvc.c")
        .include("src")
        // .shared_flag(true)
        .compile("libmygvc.a");
}
