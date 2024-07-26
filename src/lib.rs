#![no_std]

use log::{Metadata, Record, SetLoggerError};

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "embedded-logging")]
use log::Level;

#[cfg(feature = "embedded-logging")]
mod write_to;

#[cfg(feature = "embedded-logging")]
#[link(name = "app")]
extern "C" {
    // u8 is a lie but same binary definition
    pub fn embedded_logging_log(level: u8, message: *const u8);
}

struct EmbeddedLogger;

impl log::Log for EmbeddedLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            #[allow(clippy::needless_return)]
            return;
        }

        #[cfg(feature = "embedded-logging")]
        {
            let level = match record.level() {
                Level::Error => 1,
                Level::Warn => 2,
                Level::Info => 3,
                Level::Debug => 4,
                Level::Trace => 4,
            };

            let mut buffer = [0u8; 256];
            if let Ok(out) = write_to::show(&mut buffer, format_args!("{}", record.args())) {
                unsafe { embedded_logging_log(level, out.as_ptr()) }
            }
        }

        #[cfg(feature = "std")]
        {
            std::println!("{} - {}", record.level(), record.args())
        }
    }

    fn flush(&self) {}
}

static LOGGER: EmbeddedLogger = EmbeddedLogger;

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_max_level(log::LevelFilter::Debug);
    log::set_logger(&LOGGER)
}
