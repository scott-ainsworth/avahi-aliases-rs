#![warn(clippy::all)]

use thiserror::Error;

/// LoggingError enumerates all errors returned by this module
#[derive(Error, Debug)]
pub enum LoggingError {
    /// Represents a failure to connect to syslog.
    #[error("Could not connect to syslog")]
    SyslogError { source: syslog::Error },

    /// Represents a failure setting the logger (syslog only)
    #[error("Could not set logger")]
    SetLoggerError { source: log::SetLoggerError },
}

/// Initialize console logging
pub fn init_console_logging(verbose: bool, debug: bool) {
    env_logger::builder()
        .format_target(false)
        .format_level(false)
        .format_module_path(false)
        .format_timestamp(None)
        .filter(None, compute_log_level(verbose, debug))
        .init();
}

pub fn init_syslog_logging(verbose: bool, debug: bool) -> Result<(), LoggingError> {
    let formatter = syslog::Formatter3164 {
        facility: syslog::Facility::LOG_DAEMON,
        hostname: None,
        process: "avahi-alias-daemon".into(),
        pid: sysinfo::get_current_pid().unwrap(),
    };
    let logger = syslog::BasicLogger::new(
        syslog::unix(formatter).map_err(|source| LoggingError::SyslogError { source })?,
    );
    log::set_boxed_logger(Box::new(logger))
        .map_err(|source| LoggingError::SetLoggerError { source })?;
    log::set_max_level(compute_log_level(verbose, debug));
    Ok(())
}

/// Set the log level based on command line arguments
fn compute_log_level(verbose: bool, debug: bool) -> log::LevelFilter {
    if debug {
        log::LevelFilter::Debug
    } else if verbose {
        log::LevelFilter::Info
    } else {
        log::LevelFilter::Warn
    }
}

#[cfg(test)]
//**********************************************************************************************
// Unit tests
//**********************************************************************************************

mod tests {

    use log;

    #[test]
    fn compute_log_level_yields_warn_as_default() {
        assert_eq!(super::compute_log_level(false, false), log::LevelFilter::Warn);
    }

    #[test]
    fn compute_log_level_yields_info_for_verbose() {
        assert_eq!(super::compute_log_level(true, false), log::LevelFilter::Info);
    }

    #[test]
    fn compute_log_level_yields_debug_for_debug() {
        assert_eq!(super::compute_log_level(false, true), log::LevelFilter::Debug);
        assert_eq!(super::compute_log_level(true, true), log::LevelFilter::Debug);
    }
}
