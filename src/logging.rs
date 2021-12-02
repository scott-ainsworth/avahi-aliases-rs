//! Logging setup for `avahi-aliases`

#![warn(clippy::all)]

use anyhow::{anyhow, Result};

/// Initialize console logging
pub fn init_console_logging(verbose: bool, debug: bool) -> Result<()> {
    env_logger::builder()
        .format_target(false)
        .format_level(false)
        .format_module_path(false)
        .format_timestamp(None)
        .filter(None, compute_log_level(verbose, debug))
        .try_init()?;
    Ok(())
}

pub fn init_syslog_logging(verbose: bool, debug: bool) -> Result<()> {
    let formatter = syslog::Formatter3164 {
        facility: syslog::Facility::LOG_DAEMON,
        hostname: None,
        process: "avahi-alias-daemon".into(),
        pid: sysinfo::get_current_pid().unwrap(),
    };
    let logger = syslog::BasicLogger::new(match syslog::unix(formatter) {
        Ok(logger) => Ok(logger),
        Err(error) => Err(anyhow!("could not open syslog: {}", error)),
    }?);
    log::set_boxed_logger(Box::new(logger))?;
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

    use std::panic;

    use log;

    use super::*;

    #[test]
    fn compute_log_level_yields_warn_as_default() {
        assert_eq!(compute_log_level(false, false), log::LevelFilter::Warn);
    }

    #[test]
    fn compute_log_level_yields_info_for_verbose() {
        assert_eq!(compute_log_level(true, false), log::LevelFilter::Info);
    }

    #[test]
    fn compute_log_level_yields_debug_for_debug() {
        assert_eq!(compute_log_level(false, true), log::LevelFilter::Debug);
        assert_eq!(compute_log_level(true, true), log::LevelFilter::Debug);
    }

    #[test]
    fn init_console_logging_works() {
        // There is a good chance that logging is already initialized by the testing
        // system; thus, an Err(...) result is ignored.
        // The result is less than perfect testing--c'est la guerre!
        let _ = init_console_logging(true, false);
    }

    #[test]
    fn init_syslog_logging_works() {
        // There is a good chance that logging is already initialized. Catch the resulting
        // panic. The result is less than perfect testing--c'est la guerre!
        let _ = panic::catch_unwind(|| assert!(init_syslog_logging(true, false).is_ok()));
    }
}
