#![warn(clippy::all)]

use structopt::lazy_static::lazy_static;
use regex::Regex;

use crate::ErrorWrapper;

/// validate an alias
pub fn validate_alias(alias: &str) -> Result<(), ErrorWrapper> {
    lazy_static! {
        static ref VALIDATION_RE: Regex = Regex::new("^[a-z0-9-]*\\.local$").unwrap();
    }
    match VALIDATION_RE.is_match(alias) {
        true => Ok(()),
        false => Err(ErrorWrapper::invalid_alias_error(alias)),
    }
}
