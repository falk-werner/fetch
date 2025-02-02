use log::{set_logger, set_max_level, Level, LevelFilter, Log};

use crate::args::Args;

struct Logger;

impl Log for Logger
{

    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{}: {}", record.level().as_str().to_lowercase(), record.args());
        }
    }

    fn flush(&self) {
        
    }
}

static LOGGER: Logger = Logger { };

pub fn init_logger(args: &Args) {
    set_logger(&LOGGER).unwrap();

    let mut level = LevelFilter::Warn;
    if args.verbose {
        level = LevelFilter::Info;
    }
    if args.silent {
        level = LevelFilter::Off;
        if args.show_error {
            level = LevelFilter::Error;
        }    
    }

    set_max_level(level);
}