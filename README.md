# SSVM automaton displayer
## Goal
The goal of this project is to convert a regex to an automaton and display it.

## Technos
The project uses SSVM to compile the Rust code to WebAssembly and call it from the nodejs front-end.

The Rust back-end uses the [rustomaton crate](https://crates.io/crates/rustomaton) to handle the conversion from regex to automaton and then from automaton to [Graphviz's dot language](https://graphviz.org/doc/info/lang.html). Then the [Graphviz C library](https://gitlab.com/graphviz/graphviz/) is used to convert the dot representation of the automaton to SVG.

The initial goal of the project was to have the whole Rust code compiled to WebAssembly but I realised that I could neither execute a command (actually [you can](https://opensource.com/article/19/4/command-line-playgrounds-webassembly) but this doesn't seem feasible with SSVM) nor [link a C library and call its functions](https://rustwasm.github.io/docs/book/reference/which-crates-work-with-wasm.html#things-a-crate-might-do-that-wont-work-with-webassembly) from WebAssembly. I decided to split my Rust code in two: a first part (compiled to WebAssembly) would convert the regex to the dot syntax and a second part would just be a server awaiting dot representation and sending back the SVG representation.

## Second State Virtual Machine
<https://www.secondstate.io/ssvm/>

## Why this project
The reason for this project is to [get a free raspberry pi 0 kit](https://www.secondstate.io/articles/raspberry-pi-for-free-20200709/) using the second state virtual machine.

## Use
You can clone this project, follow the instructions on <https://www.secondstate.io/articles/setup-rust-nodejs/#manual-setup> to install Rust (rustup, rustc, cargo), node and SSVM.

Then you can run `make run` to launch the local server and the client, and go on `localhost:3000` to use the client.

*Note that the project currently doesn't run this way successfully.*

At the end I started to have some fun with [`clap`](https://crates.io/crates/clap), a Rust crate to - amongst other things - parse command line arguments.

A command line interface is given, simply run `cargo run -- display [--regex <REGEX>]` to launch it. *--regex <REGEX>* is an optional argument, if it is not given then a regex is expected on stdin. The SVG is written on stdout, thus it can easily be piped to a tool such as [display](https://imagemagick.org/script/display.php).

```shell
cargo run -- display --regex "a*b|b.c|d?e+" | display
```

Run `cargo run -- -h` to get the help.
