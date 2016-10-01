#![feature(custom_attribute, custom_derive, plugin, question_mark, specialization, trace_macros, log_syntax)]
#![plugin(clippy, serde_macros)]
trace_macros!(true);

#[macro_use]
extern crate log;
extern crate term;
#[macro_use]
extern crate clap;

use log::{LogLevelFilter};
use clap::{App, Arg, SubCommand};

mod lib;
mod logger;

fn main() {
    let matches = App::new("Desert Server")
        .version(crate_version!())
        .author("Jan J. <farodin91@googlemail.com>")
        .about("A basic Rest server!")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("run")
            .about("controls testing features"))
        .get_matches();
    let level = match matches.occurrences_of("v") {
        0 => LogLevelFilter::Info,
        1 => LogLevelFilter::Debug,
        _ => LogLevelFilter::Trace,
    };
    logger::init(level).unwrap();

    match matches.subcommand() {
        ("run", _) => {},
        _ => println!("{}", matches.usage()),
    }
}
