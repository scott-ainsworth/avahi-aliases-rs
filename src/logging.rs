#![warn(clippy::all)]

/// Initialize logging
pub fn init_console(verbose: bool, debug: bool) {
    env_logger::builder()
        .format_target(false)
        .format_level(false)
        .format_module_path(false)
        .format_timestamp(None)
        .filter(None, compute_log_level(verbose, debug))
        .init();
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
