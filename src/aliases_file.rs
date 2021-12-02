//! Read, write, and modify an Avahi aliases file

#![warn(clippy::all)]

use std::io::{BufWriter, Read, Write};
use std::{self, fs, str};

use crate::alias::{self, Alias};
use crate::error::ErrorWrapper;
use crate::Line;

/// An Avahi aliases file.
///
/// The `AliasFile` struct encapsulates the details of managing the aliases file
/// and the aliases it contains. It provides a high-level interface to load, add,
/// and remove aliases.
#[derive(Debug)]
pub struct AliasesFile {
    file_name: String, // cov(skip)
    lines: Vec<Line>,  // cov(skip)
}

impl<'a> AliasesFile {
    /// Return a vector containing the aliases.
    /// Note: this function returns both valid and invalid aliases.
    pub fn all_aliases(&self) -> Vec<Alias<'_>> {
        self.lines.iter().filter_map(|line| line.alias()).collect()
    }

    /// Return a vector of valid aliases.
    pub fn aliases(&self) -> Vec<&str> {
        self.lines.iter().filter_map(|line| line.alias().map(|a| a.ok()).flatten()).collect()
    }

    /// Return a vector of invalid aliases.
    pub fn invalid_aliases(&self) -> Vec<&str> {
        self.lines.iter().filter_map(|line| line.alias().map(|a| a.err()).flatten()).collect()
    }

    /// Return the number of aliases
    pub fn alias_count(&self) -> usize { self.aliases().len() }

    pub fn from_file(filename: &str, allow_invalid: bool) -> Result<Self, ErrorWrapper> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(filename)
            .map_err(|error| ErrorWrapper::new_open_error(filename, error))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|error| ErrorWrapper::new_read_error(filename, error))?;
        // Build the AliasesFile
        let aliases_file = AliasesFile {
            file_name: filename.to_owned(),
            lines: buf.lines().map(|text| Line::new(text.to_owned())).collect(),
        };
        if allow_invalid || aliases_file.all_aliases_are_valid() {
            Ok(aliases_file)
        } else {
            Err(ErrorWrapper::new_invalid_alias_file_error(
                filename,
                aliases_file.invalid_aliases()[0],
            ))
        }
    }

    pub fn append(&self, aliases: &[&str]) -> Result<(), ErrorWrapper> {
        alias::validate_aliases(aliases)?;
        let mut writer = fs::OpenOptions::new()
            .append(true)
            .open(&self.file_name)
            .map(BufWriter::new)
            .map_err(|error| ErrorWrapper::new_open_error(&self.file_name, error))?;
        for alias in aliases {
            writer
                .write_all(format!("{}\n", alias).as_bytes())
                .map_err(|error| ErrorWrapper::new_write_error(&self.file_name, error))?;
        }
        Ok(())
    }

    pub fn remove(&self, aliases: &[&str], force: bool) -> Result<(), ErrorWrapper> {
        if !force {
            alias::validate_aliases(aliases)?;
        }
        let mut writer = fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(&self.file_name)
            .map(BufWriter::new)
            .map_err(|error| ErrorWrapper::new_open_error(&self.file_name, error))?;
        let retained_lines = (&self.lines).iter().filter(|line| match line.alias() {
            Some(Err(_)) => false,                        // Invalid aliases must go!
            Some(Ok(alias)) => !aliases.contains(&alias), // Remove specified aliases
            _ => true,                                    // everything else stays
        });
        for line in retained_lines {
            writer
                .write_all(format!("{}\n", line.text()).as_bytes())
                .map_err(|error| ErrorWrapper::new_write_error(&self.file_name, error))?;
        }
        Ok(())
    }

    pub fn all_aliases_are_valid(&self) -> bool { self.invalid_aliases().is_empty() }

    /// Used for testing.
    #[allow(dead_code)]
    fn lines(&self) -> &Vec<Line> { &self.lines }
}

//**********************************************************************************************
// Unit Tests
//**********************************************************************************************

#[cfg(test)]
mod tests {

    use std::{self, fs, str};
    use std::io::{BufWriter, Error, Write};

    use crate::ErrorWrapper;
    use super::AliasesFile;

    const FILE_HEADER: &str = "# This is a unit test temporary file";
    const VALID_ALIASES: [&str; 5] = ["a.local", "b.local", "c.local", "d.local", "e.local"];
    const INVALID_ALIASES: [&str; 5] = [".local", "x .local", "x*.local", "-.local", "*.local"];

    /// Ensure a correct avahi-aliases file loads.
    #[test]
    fn from_file_loads_expected_content() {
        let fn_name = stringify!(from_file_loads_expected_content);
        for n in 0..5 {
            let test_file = TestFile::new(fn_name, n, true);
            let aliases_file = AliasesFile::from_file(&test_file.file_name, true).unwrap();
            assert_eq!(aliases_file.lines()[0].text(), FILE_HEADER);
            for i in 0..n {
                assert_eq!(aliases_file.lines()[i + 1].text(), VALID_ALIASES[i]);
            }
            assert_eq!(aliases_file.lines()[n + 1].text(), INVALID_ALIASES[n]);
        }
    }

    /// Ensure the alias count is correct.
    #[test]
    fn alias_count_returns_correct_count() {
        let fn_name = stringify!(alias_count_returns_correct_count);
        for n in 0..5 {
            let test_file = TestFile::new(fn_name, n, false);
            let aliases_file = AliasesFile::from_file(&test_file.file_name, false).unwrap();
            assert_eq!(aliases_file.lines().len(), n + 1);
            assert_eq!(aliases_file.alias_count(), n);
        }
    }

    /// Ensure an incorrect (contains invalid aliases) avahi-aliases does not load
    /// when the `invalid_alias` parameter is `false`.
    #[test]
    fn load_fails_when_invalid_alias_not_allowed() {
        let fn_name = stringify!(load_fails_when_invalid_alias_not_allowed);
        for n in 1..5 {
            let test_file = TestFile::new(fn_name, n, true);
            let aliases_file = AliasesFile::from_file(&test_file.file_name, false);
            assert!(aliases_file.is_err());
        }
    }

    /// Ensure `aliases` returns only the valid aliases.
    #[test]
    fn aliases_returns_expected_aliases() {
        let fn_name = stringify!(aliases_returns_expected_aliases);
        for n in 0..5 {
            let test_file = TestFile::new(fn_name, n, true);
            let aliases_file = AliasesFile::from_file(&test_file.file_name, true).unwrap();
            for i in 0..n {
                assert_eq!(aliases_file.all_aliases()[i].unwrap(), VALID_ALIASES[i]);
            }
        }
    }

    /// Ensure `all_aliases` returns the valid and invalid aliases.
    #[test]
    fn all_aliases_returns_expected_aliases() {
        let fn_name = stringify!(all_aliases_returns_expected_aliases);
        for n in 0..5 {
            let test_file = TestFile::new(fn_name, n, true);
            let aliases_file = AliasesFile::from_file(&test_file.file_name, true).unwrap();
            for i in 0..n {
                assert_eq!(aliases_file.all_aliases()[i].unwrap(), VALID_ALIASES[i]);
            }
            assert_eq!(aliases_file.all_aliases()[n].unwrap_err(), INVALID_ALIASES[n]);
        }
    }

    /// Ensure an incorrect (contains invalid aliases) avahi-aliases loads
    /// when the `invalid_alias` parameter is `true`.
    #[test]
    fn load_succeeds_when_invalid_alias_allowed() {
        let fn_name = stringify!(load_succeeds_when_invalid_alias_allowed);
        for n in 1..5 {
            let test_file = TestFile::new(fn_name, n, true);
            let aliases_file = AliasesFile::from_file(&test_file.file_name, true);
            assert!(aliases_file.is_ok());
        }
    }

    /// Ensure the append function appends the specified aliases.
    #[test]
    fn append_appends() {
        let fn_name = stringify!(append_appends);
        for n in 0..5 {
            let test_file = TestFile::new(fn_name, n, false);
            let aliases_file = AliasesFile::from_file(&test_file.file_name, false).unwrap();
            aliases_file
                .append(&vec!["b0.local"])
                .unwrap_or_else(|error| panic!("Append failed: {}", error));
            let aliases_file = AliasesFile::from_file(&test_file.file_name, false).unwrap();
            let aliases = aliases_file.aliases();
            assert_eq!(aliases[n], "b0.local");
        }
    }

    /// Ensure the remove function renews the specified aliases.
    #[test]
    fn remove_removes() -> Result<(), Error> {
        let fn_name = stringify!(remove_removes);
        for n in 0..5 {
            let test_file = TestFile::new(fn_name, n, false);
            let aliases_file = AliasesFile::from_file(&test_file.file_name, false).unwrap();
            if n >= 2 {
                aliases_file
                    .remove(&[VALID_ALIASES[0], VALID_ALIASES[2]], false)
                    .unwrap_or_else(|error| panic!("Remove failed: {}", error));
            } else if n > 0 {
                aliases_file
                    .remove(&[VALID_ALIASES[0]], false)
                    .unwrap_or_else(|error| panic!("Remove failed: {}", error));
            }
            let aliases_file = AliasesFile::from_file(&test_file.file_name, false).unwrap();
            let aliases = aliases_file.aliases();
            if n > 3 {
                assert_eq!(aliases[0], VALID_ALIASES[1]);
                assert_eq!(aliases[1], VALID_ALIASES[3]);
            } else if n > 1 {
                assert_eq!(aliases[0], VALID_ALIASES[1]);
            }
        }
        Ok(())
    }

    /// Ensure remove --force removes invalid aliases.
    #[test]
    fn remove_force_removes_invalid_alias_in_avahi_aliases_file() -> Result<(), ErrorWrapper> {
        let fn_name = stringify!(remove_force_removes_invalid_alias_in_avahi_aliases_file);
        for n in 1..5 {
            let test_file = TestFile::new(fn_name, n, true);
            let aliases_file = AliasesFile::from_file(&test_file.file_name, true).unwrap();
            aliases_file.remove(&[], false)?;
            let aliases_file = AliasesFile::from_file(&test_file.file_name, false);
            assert!(aliases_file.is_ok());
        }
        Ok(())
    }

    /// Create and remove (using Drop trait) test files
    struct TestFile {
        file_name: String,
    }

    impl<'a> TestFile {
        fn new(file_name: &'static str, line_count: usize, include_invalid: bool) -> TestFile {
            let file_name = format!("data/{}-{}.txt", file_name, line_count);
            fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&file_name)
                .map(BufWriter::new)
                .and_then(|mut writer| {
                    writer.write_all(format!("{}\n", FILE_HEADER).as_bytes())?;
                    for i in 0..line_count {
                        writer.write_all(format!("{}\n", VALID_ALIASES[i]).as_bytes())?;
                    }
                    if include_invalid {
                        writer.write_all(
                            format!("{}\n", INVALID_ALIASES[line_count]).as_bytes(),
                        )?;
                    }
                    Ok(())
                })
                .unwrap_or_else(|_| {
                    panic!(
                        "Could not create test file: cwd={:?}, file={:?}",
                        std::env::current_dir(),
                        file_name
                    )
                });
            TestFile { file_name }
        }
    }

    impl Drop for TestFile {
        fn drop(&mut self) {
            fs::remove_file(&self.file_name).unwrap_or_else(|_| {
                panic!(
                    "Could not delete test file: cwd={:?}, file={:?}",
                    std::env::current_dir(),
                    self.file_name
                )
            });
        }
    }
}

// end
