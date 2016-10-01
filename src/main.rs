#[macro_use]
extern crate log;
extern crate term;
extern crate clap;

use log::{LogLevelFilter};
use clap::app::App;

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
        .subcommand(SubCommand::with_name("test")
            .about("controls testing features"))
        .get_matches();
    let level = match matches.occurrences_of("v") {
        0 => LogLevelFilter::Info,
        1 => LogLevelFilter::Debug,
        _ => LogLevelFilter::Trace,
    };

    logger::init(level).unwrap();
    error!("Hello World!");
    warn!("Hello World!");
    info!("Hello World! {:?}", matches);
    debug!("Hello World!");
    trace!("Hello World!");
}
