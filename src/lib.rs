#![no_std]

use log::{Metadata, Record, SetLoggerError};

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "embedded-logging")]
mod write_to;

#[cfg(feature = "embedded-logging")]
#[link(name = "app")]
extern "C" {
    // u8 is a lie but same binary definition
    pub fn embedded_logging_log(level: u8, message: *const u8);
}

#[repr(u8)]
pub enum EmbeddedLogLevel {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

impl From<log::Level> for EmbeddedLogLevel {
    fn from(level: log::Level) -> Self {
        match level {
            log::Level::Error => EmbeddedLogLevel::Error,
            log::Level::Warn => EmbeddedLogLevel::Warn,
            log::Level::Info => EmbeddedLogLevel::Info,
            log::Level::Debug => EmbeddedLogLevel::Debug,
            log::Level::Trace => EmbeddedLogLevel::Trace,
        }
    }
}

impl From<EmbeddedLogLevel> for log::LevelFilter {
    fn from(level: EmbeddedLogLevel) -> Self {
        match level {
            EmbeddedLogLevel::Error => log::LevelFilter::Error,
            EmbeddedLogLevel::Warn => log::LevelFilter::Warn,
            EmbeddedLogLevel::Info => log::LevelFilter::Info,
            EmbeddedLogLevel::Debug => log::LevelFilter::Debug,
            EmbeddedLogLevel::Trace => log::LevelFilter::Trace,
        }
    }
}

impl From<EmbeddedLogLevel> for u8 {
    fn from(level: EmbeddedLogLevel) -> Self {
        match level {
            EmbeddedLogLevel::Error => 1,
            EmbeddedLogLevel::Warn => 2,
            EmbeddedLogLevel::Info => 3,
            EmbeddedLogLevel::Debug => 4,
            EmbeddedLogLevel::Trace => 5,
        }
    }
}

impl From<u8> for EmbeddedLogLevel {
    fn from(level: u8) -> Self {
        match level {
            1 => EmbeddedLogLevel::Error,
            2 => EmbeddedLogLevel::Warn,
            3 => EmbeddedLogLevel::Info,
            4 => EmbeddedLogLevel::Debug,
            5 => EmbeddedLogLevel::Trace,
            _ => panic!("Unsupported log level"),
        }
    }
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
            let level = EmbeddedLogLevel::from(record.level());

            let mut buffer = [0u8; 256];
            if let Ok(out) = write_to::show(&mut buffer, format_args!("{}", record.args())) {
                unsafe { embedded_logging_log(level.into(), out.as_ptr()) }
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

pub fn init_logger(max_log_level: EmbeddedLogLevel) -> Result<(), SetLoggerError> {
    log::set_max_level(max_log_level.into());
    log::set_logger(&LOGGER)
}
