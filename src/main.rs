// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's full 'builder pattern' style of creating arguments which is
// more verbose, but allows easier editing, and at times more advanced options, or the possibility
// to generate arguments dynamically.
extern crate clap;
use clap::{App, Arg, SubCommand};

mod cmd;

fn main() {
    let matches = App::new("hetero-tools")
        .version("0.1.0")
        .author("Carl P. <carl.w.pearson@gmail.com>")
        .about("")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("analyze")
                .about("analyze profiles")
                .version("0.1.0")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("analyze") {
        if matches.is_present("INPUT") {
            let path_str = matches.value_of("INPUT").unwrap();
            eprintln!("Using input file: {}", path_str);
            cmd::analyze::run(path_str);
        } else {
            println!("Printing normally...");
        }
    }

    // more program logic goes here...
}
