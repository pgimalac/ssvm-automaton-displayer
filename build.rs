extern crate cc;

fn main() {
    // println!("cargo:rustc-link-search=/usr/lib/");
    // println!("cargo:rustc-link-lib=mygvc");
    // println!("cargo:rustc-link-lib=gvc");
    // println!("cargo:rustc-link-lib=cgraph");

    cc::Build::new()
        .file("src/c/mygvc.c")
        .include("/usr/include")
        .archiver("/usr/bin/ar")
        .compile("libmygvc.a");
}
