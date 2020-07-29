use std::ffi::CStr;
use rustomaton::{dfa::ToDfa,nfa::ToNfa,regex::Regex};
use std::{str::FromStr, os::raw::{c_char, c_int}};
use wasm_bindgen::prelude::*;

extern {
    fn dot_to_svg(_: c_char, _: c_int) -> c_char;
    fn c_free(_: c_char);
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
    let mut svg = unsafe { dot_to_svg(std::ffi::CString::new(dot).unwrap().as_ptr() as _, len as _) };

    if svg == 0 {
        return String::new();
    }

    let ret = unsafe { CStr::from_ptr(&mut svg as _) }.to_str().expect("").to_string();
    unsafe { c_free(svg) };

    ret
}
