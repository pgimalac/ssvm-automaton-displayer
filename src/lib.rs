use rustomaton::automaton::Automata;
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
pub fn regex_to_svg(reg: &str, negate: u8, determinize: u8, minimize: u8, complete: u8) -> String {
    let negate = negate != 0;
    let determinize = determinize != 0;
    let minimize = minimize != 0;
    let complete = complete != 0;

    let mut aut = match Regex::from_str(reg) {
        Ok(regex) => regex.to_nfa(),
        _ => return String::new(),
    };

    if negate {
        aut = aut.negate();
    }

    let dot = if determinize {
        let mut aut = if minimize {
            aut.to_dfa().minimize()
        } else {
            aut.to_dfa()
        };

        if complete {
            aut = aut.complete();
        }

        aut.to_dot()
    } else if complete {
        aut.complete().to_dot()
    } else {
        aut.to_dot()
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
