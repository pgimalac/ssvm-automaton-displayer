use rustomaton::{dfa::ToDfa, nfa::ToNfa, regex::Regex};
use std::ffi::CStr;
use std::ptr::null;
use std::{
    os::raw::{c_char, c_int},
    str::FromStr,
};
use wasm_bindgen::prelude::*;

#[link(name = "mygvc")]
#[link(name = "gvc")]
#[link(name = "cgraph")]
extern "C" {
    fn dot_to_svg(_: *const c_char, _: c_int) -> *const c_char;
    fn c_free(_: *const c_char);
}

#[wasm_bindgen]
pub fn regex_to_svg(s: &str, determinize: u8, minimize: u8) -> String {
    let dot = match Regex::from_str(s) {
        Ok(regex) => {
            if determinize != 0 {
                if minimize != 0 {
                    regex.to_dfa().minimize()
                } else {
                    regex.to_dfa()
                }
                .to_dot()
            } else {
                regex.to_nfa().to_dot()
            }
        }
        _ => return String::new(),
    };

    let len = dot.len();
    let dotstring = std::ffi::CString::new(dot).unwrap();
    let svg = unsafe { dot_to_svg(dotstring.as_ptr() as _, len as _) };

    if svg == null() {
        return String::new();
    }

    let ret = unsafe { CStr::from_ptr(svg) }
        .to_string_lossy()
        .into_owned();

    unsafe { c_free(svg) };

    ret
}
