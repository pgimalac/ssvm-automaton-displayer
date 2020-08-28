boost-libs boost libstdc++5

rustup target add wasm32-wasi
rustup component add wasm32-wasi
rustup update

node node/app.js
terminate called after throwing an instance of 'std::filesystem::__cxx11::filesystem_error'
  what():  filesystem error: directory iterator cannot open directory: No such file or directory [/usr/lib/x86_64-linux-gnu]

sudo ln -s /usr/lib /usr/lib/x86_64-linux-gnu

node node/app.js
ok



error: failed to generate bindings for `regex_to_svg`

Caused by:
    Type of `determinize` is Bool, only Integer, String or Vector<u8> are supported now



```
    let mut cmd = Command::new("dot")
        .arg("-Tsvg")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    std::io::copy(&mut dot.as_bytes(), &mut cmd.stdin.as_mut().unwrap()).unwrap();

    return String::from_utf8_lossy(cmd.wait_with_output().unwrap().stdout.as_slice()).into_owned();
```

thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: Custom { kind: Other, error: "operation not supported on wasm yet" }', src/lib.rs:28:19
(=> Command::new)

```Rust
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

    let mut cmd = Command::new("dot")
        .arg("-Tsvg")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    std::io::copy(&mut dot.as_bytes(), &mut cmd.stdin.as_mut().unwrap()).unwrap();

    return String::from_utf8_lossy(cmd.wait_with_output().unwrap().stdout.as_slice()).into_owned();
}
```


the trait `wasm_bindgen::convert::IntoWasmAbi` is not implemented for `std::result::Result<std::string::String, ()>`




```Rust
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
    let dotstring = std::ffi::CString::new(dot).unwrap();
    let mut svg = unsafe { dot_to_svg(dotstring.as_ptr() as _, len as _) };

    if svg == 0 {
        return String::new();
    }

    let ret = unsafe { CStr::from_ptr(&mut svg as _) }.to_str().expect("").to_string();
    unsafe { c_free(svg) };

    ret
}
```


sudo pacman -S lib32-glibc


TcpStream connect =>
operation not supported on wasm yet
