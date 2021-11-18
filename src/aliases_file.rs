#![warn(clippy::all)]

use std::io::{BufWriter, Read, Write};
use std::{self, fs, str};

use crate::alias::{self, Alias};
use crate::error::ErrorWrapper;
use crate::Line;

#[derive(Debug)]
pub struct AliasesFile {
    file_name: String,
    lines: Vec<Line>,
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

    /// Ensure a correct avahi-aliases file loads.
    #[test]
    fn from_file_loads_expected_content() {
        for n in 0..5 {
            let test_file = TestFile::new("data/test-avahi-aliases.txt", n, &[]);
            let aliases_file = AliasesFile::from_file(test_file.file_name, false).unwrap();
            assert_eq!(aliases_file.lines().len(), n);
            for i in 0..n {
                assert_eq!(aliases_file.lines()[i].text(), format!("a{}.local", i));
            }
        }
    }

    /// Ensure an incorrect (contains invalid aliases) avahi-aliases does not load
    /// when the `invalid_alias` parameter is `false`.
    #[test]
    fn load_fails_when_invalid_alias_not_allowed() {
        for n in 1..5 {
            let test_file =
                TestFile::new("data/avahi-alias-append-invalid-not-allowed.txt", n, &[n - 1]);
            let aliases_file = AliasesFile::from_file(test_file.file_name, false);
            eprintln!("**** aliases_file = {:?}", aliases_file);
            assert!(aliases_file.is_err());
        }
    }

    /// Ensure an incorrect (contains invalid aliases) avahi-aliases loads
    /// when the `invalid_alias` parameter is `true`.
    #[test]
    fn load_succeeds_when_invalid_alias_allowed() {
        for n in 1..5 {
            let test_file =
                TestFile::new("data/avahi-alias-append-invalid-allowed.txt", n, &[n - 1]);
            let aliases_file = AliasesFile::from_file(test_file.file_name, true);
            eprintln!("**** aliases_file = {:?}", aliases_file);
            assert!(aliases_file.is_ok());
        }
    }

    /// Ensure the append function appends the specified aliases.
    #[test]
    fn append_appends() {
        for n in 0..5 {
            let test_file = TestFile::new("data/avahi-aliases-append.txt", n, &[]);
            let aliases_file = AliasesFile::from_file(test_file.file_name, false).unwrap();
            aliases_file
                .append(&vec!["b0.local"])
                .unwrap_or_else(|error| panic!("Append failed: {}", error));
            let aliases_file = AliasesFile::from_file(test_file.file_name, false).unwrap();
            let aliases = aliases_file.aliases();
            assert_eq!(aliases[n], "b0.local");
        }
    }

    /// Ensure the remove function renews the specified aliases.
    #[test]
    fn remove_removes() -> Result<(), Error> {
        for n in 0..5 {
            let test_file = TestFile::new("data/avahi-aliases-remove.txt", n, &[]);
            let aliases_file = AliasesFile::from_file(test_file.file_name, false).unwrap();
            if n >= 2 {
                aliases_file
                    .remove(&["a0.local", "a2.local"], false)
                    .unwrap_or_else(|error| panic!("Remove failed: {}", error));
            } else if n > 0 {
                aliases_file
                    .remove(&["a0.local"], false)
                    .unwrap_or_else(|error| panic!("Remove failed: {}", error));
            }
            let aliases_file = AliasesFile::from_file(test_file.file_name, false).unwrap();
            let aliases = aliases_file.aliases();
            if n > 3 {
                assert_eq!(aliases[0], "a1.local");
                assert_eq!(aliases[1], "a3.local");
            } else if n > 1 {
                assert_eq!(aliases[0], "a1.local");
            }
        }
        Ok(())
    }

    /// Ensure remove --force removes invalid aliases.
    #[test]
    fn remove_force_removes_invalid_alias_in_avahi_aliases_file() -> Result<(), ErrorWrapper> {
        for n in 1..5 {
            let test_file = TestFile::new("data/avahi-aliases-remove-force.txt", n, &[n - 1]);
            let aliases_file = AliasesFile::from_file(test_file.file_name, true).unwrap();
            aliases_file.remove(&[], false)?;
            let aliases_file = AliasesFile::from_file(test_file.file_name, false);
            assert!(aliases_file.is_ok());
        }
        Ok(())
    }

    /// Create and remove (using Drop trait) test files
    struct TestFile {
        file_name: &'static str,
    }

    impl<'a> TestFile {
        fn new(
            file_name: &'static str, line_count: usize, invalid_lines: &[usize],
        ) -> TestFile {
            fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_name)
                .map(BufWriter::new)
                .and_then(|mut writer| {
                    for i in 0..line_count {
                        writer.write_all(format!("a{}.local\n", i).as_bytes())?;
                        if invalid_lines.contains(&i) {
                            static INVALID_ALIASES: [&str; 5] =
                                ["", "xyzzy ", "xyzz*", "-", "*.*"];
                            writer.write_all(
                                format!("{}.local\n", INVALID_ALIASES[i]).as_bytes(),
                            )?;
                        }
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
            fs::remove_file(self.file_name).unwrap_or_else(|_| {
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
