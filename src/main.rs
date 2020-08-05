extern crate clap;

use crate::lib::regex_to_svg;
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, SubCommand,
};
use std::io::{
    BufRead, Error,
    ErrorKind::{Other, UnexpectedEof},
};

mod lib;

fn main() -> Result<(), Error> {
    let matches = app_from_crate!()
        .subcommand(
            SubCommand::with_name("server")
                .arg(
                    Arg::with_name("daemon")
                        .short("d")
                        .long("daemon")
                        .takes_value(false)
                        .help("Runs the server as a daemon."),
                )
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .takes_value(true)
                        .validator(|val| match val.parse::<u16>() {
                            Ok(val) if val >= 49152 => Ok(()),
                            Ok(_) => Err("The port number must be over 49152.".to_string()),
                            Err(err) => Err(err.to_string()),
                        })
                        .value_name("INT")
                        .help(
                            "Specify the port used by the server, otherwise a random port is used.",
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("display")
            .about("Displays the SVG representation of a given regex transformed to an automaton and modified with the specifiers below.")
                .after_help("Note: the automaton specifiers (negate, determinze, minimize, complete) are applied in the order above.")
                .arg(
                    Arg::with_name("regex")
                        .short("r")
                        .long("regex")
                        .takes_value(true)
                        .value_name("REGEX")
                        .help("The regex that will be converted to an automaton. If not specified then a regex is expected on stdin."),
                )
                .arg(
                    Arg::with_name("negate")
                        .short("n")
                        .long("negate")
                        .help("To show a negated automaton.")
                        .display_order(1),
                )
                .arg(
                    Arg::with_name("determinize")
                        .short("d")
                        .long("determinize")
                        .help("To show a deterministic automaton.")
                        .display_order(2),
                )
                .arg(
                    Arg::with_name("minimize")
                        .short("m")
                        .long("minimize")
                        .help("To show a minimized automaton.")
                        .requires("determinize")
                        .display_order(3),
                )
                .arg(
                    Arg::with_name("complete")
                        .short("c")
                        .long("complete")
                        .help("To show a complete automaton.")
                        .display_order(4),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("display", Some(matches)) => {
            let regex = match matches.value_of("regex").map(String::from) {
                Some(val) => val,
                None => std::io::stdin()
                    .lock()
                    .lines()
                    .next()
                    .unwrap_or(Err(Error::new(
                        UnexpectedEof,
                        "Unexpected end of stdin: a regex was expected.",
                    )))?,
            };

            let svg = regex_to_svg(
                &regex,
                matches.is_present("negate") as _,
                matches.is_present("determinize") as _,
                matches.is_present("minimize") as _,
                matches.is_present("complete") as _,
            );

            if svg.is_empty() {
                eprintln!("{}", "An error occured.");
                Err(Error::new(Other, ""))
            } else {
                println!("{}", svg);
                Ok(())
            }
        }
        ("server", Some(_matches)) => {
            //TODO
            Err(Error::new(Other, "Unimplemented"))
        }
        _ => Err(Error::new(Other, "A subcommand must be given")),
    }
}
