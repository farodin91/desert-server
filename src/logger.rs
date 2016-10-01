use std::io::Write;

use log;
use log::{Log, LogRecord, LogLevel, LogMetadata, SetLoggerError, LogLevelFilter};

use term;

pub use term::color;
pub use term::Attr;

struct Logger;

impl Log for Logger {
    fn enabled(&self, _: &LogMetadata) -> bool {
        true
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            match record.level() {
                LogLevel::Error => {
                    let mut t = term::stdout().unwrap();
                    t.fg(term::color::BRIGHT_RED).unwrap();
                    t.attr(term::Attr::Bold).unwrap();
                    write!(t, "error: ").unwrap();
                    t.reset().unwrap();
                    writeln!(t, "{}", record.args()).unwrap();
                    t.reset().unwrap();
                },
                LogLevel::Warn => {
                    let mut t = term::stdout().unwrap();
                    t.fg(term::color::BRIGHT_YELLOW).unwrap();
                    t.attr(term::Attr::Bold).unwrap();
                    write!(t, "warn: ").unwrap();
                    t.reset().unwrap();
                    writeln!(t, "{}", record.args()).unwrap();
                    t.reset().unwrap();
                },
                LogLevel::Info => {
                    let mut t = term::stdout().unwrap();
                    t.attr(term::Attr::Bold).unwrap();
                    write!(t, "info: ").unwrap();
                    t.reset().unwrap();
                    writeln!(t, "{}", record.args()).unwrap();
                    t.reset().unwrap();
                },
                LogLevel::Debug => {
                    let mut t = term::stdout().unwrap();
                    t.fg(term::color::BRIGHT_MAGENTA).unwrap();
                    t.attr(term::Attr::Bold).unwrap();
                    write!(t, "debug: ").unwrap();
                    t.reset().unwrap();
                    writeln!(t, "{}", record.args()).unwrap();
                    t.reset().unwrap();
                },
                LogLevel::Trace => {
                    let mut t = term::stdout().unwrap();
                    t.fg(term::color::BRIGHT_MAGENTA).unwrap();
                    t.attr(term::Attr::Bold).unwrap();
                    write!(t, "trace: ").unwrap();
                    t.reset().unwrap();
                    writeln!(t, "{}", record.args()).unwrap();
                    t.reset().unwrap();
                },
            }
        }
    }
}

pub fn init(log_level: LogLevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(log_level);
        Box::new(Logger)
    })
}
