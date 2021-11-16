#![warn(clippy::all)]

use structopt::lazy_static::lazy_static;
use regex::Regex;

pub type Alias<'a> = Result<&'a str, &'a str>;

pub fn is_valid(alias: &str) -> bool {
    lazy_static! {
        static ref VALIDATION_RE: Regex = Regex::new("^[a-z0-9-]*\\.local$").unwrap();
    }
    VALIDATION_RE.is_match(alias)
}

pub fn new(alias: &str) -> Alias<'_> {
    lazy_static! {
        static ref VALIDATION_RE: Regex = Regex::new("^[a-z0-9-]+\\.local$").unwrap();
    }
    match is_valid(alias) {
        true => Ok(alias),
        false => Err(alias),
    }
}

#[macro_export]
macro_rules! validate_aliases {
    ( $aliases:ident ) => {
        for alias in $aliases.iter() {
            if !crate::alias::is_valid(&alias) {
                return Err(ErrorWrapper::invalid_alias_error(alias))
            }
        }
    };
}

//**********************************************************************************************
// Unit tests
//**********************************************************************************************

#[cfg(test)]
mod tests {
    use crate::ErrorWrapper;
    use super::{is_valid, new};

    static VALID_ALIASES: [&str; 3] = ["a.local", "xyzzy.local", "b0.local"];
    static INVALID_ALIASES: [&str; 2] = ["a. local", "xyzz*.local"];

    #[test]
    fn is_valid_returns_true_for_valid_alias() {
        VALID_ALIASES.iter().for_each(|a| assert!(is_valid(a)))
    }

    #[test]
    fn is_valid_returns_false_for_invalid_alias() {
        INVALID_ALIASES.iter().for_each(|a| assert!(!is_valid(a)))
    }

    #[test]
    fn validate_returns_ok_for_valid_alias() {
        VALID_ALIASES.iter().for_each(|a| assert!(new(a).is_ok()))
    }

    #[test]
    fn validate_returns_err_for_invalid_alias() {
        INVALID_ALIASES.iter().for_each(|a| assert!(new(a).is_err()))
    }

    #[test]
    fn validate_saves_valid_alias_as_ok() {
        VALID_ALIASES.iter().for_each(|a| assert_eq!(new(a).unwrap(), *a))
    }

    #[test]
    fn validate_saves_invalid_alias_as_err() {
        INVALID_ALIASES.iter().for_each(|a| assert_eq!(new(a).unwrap_err(), *a))
    }

    #[test]
    fn validate_aliases_returns_ok_for_valid_aliases() {
        fn macro_wrapper() -> Result<(), ErrorWrapper> {
            validate_aliases!(VALID_ALIASES);
            Ok(())
        }
        assert!(macro_wrapper().is_ok())
    }

    #[test]
    fn validate_aliases_returns_err_for_invalid_aliases() {
        fn macro_wrapper() -> Result<(), ErrorWrapper> {
            validate_aliases!(INVALID_ALIASES);
            Ok(())
        }
        assert!(macro_wrapper().is_err())
    }
}
