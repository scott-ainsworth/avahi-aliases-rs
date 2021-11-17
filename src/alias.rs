#![warn(clippy::all)]

use structopt::lazy_static::lazy_static;
use regex::Regex;

/// A `Result` used to represents an alias, valid or invalid. A valid alias is represented
/// as `Result::Ok<&str>`, where the **&str** is the alias (e.g. **gandalf.local**). An invalid
/// alias is represented as `Result::Err<&str>`, where the **&str** is the invalid alias.
///
/// # Examples
///
/// ```
/// let a1 = Alias::new("a1.local") // Ok("a1.local")
/// let a2 = Alias::new("a*.local") // Err("a*.local")
/// ```
pub type Alias<'a> = Result<&'a str, &'a str>;

/// Determine the validity of a candidate alias.
///
/// Returns `true` if `alias` is a valid alias; otherwise, returns `false`.
///
/// # Examples
///
/// ```
/// alias::is_valid("a1.local") // true
/// alias::is_valid("a*.local") // false
/// ```
///
/// # Notes
///
/// - The current definition of a valid alias is very simple: a word comprising at least one
///   letter, digit, or hyphen followed by `.local`; and, the word must begin and end with a
///   letter or digit.
pub fn is_valid(alias: &str) -> bool {
    lazy_static! {
        static ref VALIDATION_RE: Regex =
            Regex::new(r#"^[a-z0-9]([a-z0-9-]*[a-z0-9])?\.local$"#).unwrap();
    }
    VALIDATION_RE.is_match(alias)
}

/// Creates a new `Alias` from a specified alias.
///
/// # Examples
///
/// ```
/// let a1 = Alias::new("a1.local") // Ok("a1.local")
/// let a2 = Alias::new("a*.local") // Err("a*.local")
/// ```
pub fn new(alias: &str) -> Alias<'_> {
    match is_valid(alias) {
        true => Ok(alias),
        false => Err(alias),
    }
}

/// Ensures all aliases in collection all valid.
///
/// Causes the calling function to return an `Err(ErrorWrapper::InvalidAliasError)` if any
/// alias is invalid. Otherwise, the calling function continues execution.
///
/// # Examples
///
/// ```
/// fn some_action(aliases: &[&str]) -> Result<(), ErrorWrapper> {
///     validate_aliases(aliases);  // returns if any aliases are invalid
///     // execution continues if all aliases are valid
///     ...
/// }
/// ```
///
/// # Notes
/// - This macro is used to validate aliases entered on the command line; and, to stop
/// execution if invalid aliases are found.
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

    static VALID_ALIASES: [&str; 4] = ["a.local", "xyzzy.local", "b0.local", "a-z.local"];
    static INVALID_ALIASES: [&str; 5] =
        ["a. local", "xyzz*.local", ".local", "-.local", "a-.local"];

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
