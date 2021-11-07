#![warn(clippy::all)]

pub use log as msg;

/// Initialize logging
pub fn init(verbose: bool, debug: bool) {
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
    if verbose {
        log::LevelFilter::Info
    } else if debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Warn
    }
}
