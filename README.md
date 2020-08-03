# SSVM automaton displayer
## Goal
The goal of this project is to convert regexes to an automaton and display it.

## Technos
The project uses SSVM to compile the Rust back-end to WebAssembly and call it from the nodejs front-end.

The Rust back-end uses the [rustomaton crate](https://crates.io/crates/rustomaton) to handle the conversion from regex to automaton and then from automaton to [Graphviz's dot language](https://graphviz.org/doc/info/lang.html). Then the [Graphviz C library](https://gitlab.com/graphviz/graphviz/) is used to convert the dot representation of the automaton to SVG.

The front-end of the project is in nodejs and has a text field and a few checkboxes.

## Second State Virtual Machine
<https://www.secondstate.io/ssvm/>

## Why this project
The reason for this project is to [get a free raspberry pi 0 kit](https://www.secondstate.io/articles/raspberry-pi-for-free-20200709/) using the second state virtual machine.

## Use
You can clone this project, follow the instructions on <https://www.secondstate.io/articles/setup-rust-nodejs/#manual-setup>, run `ssvmup build` and then `node app/app.js`.

*Note that the project currently doesn't run this way successfully.*

A command line interface is given, simply run `cargo run [regex]` to launch it. *regex* is an optional argument, if it is not given then a regex is expected on stdin. The SVG is written on stdout, thus it can easily be piped to a tool such as [display](https://imagemagick.org/script/display.php).

```shell
cargo run "a*b|b.c|d?e+" | display
```
