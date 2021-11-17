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

    pub fn from_file(filename: &str) -> Result<Self, ErrorWrapper> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(filename)
            .map_err(|error| ErrorWrapper::new_open_error(filename, error))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|error| ErrorWrapper::new_read_error(filename, error))?;
        Ok(AliasesFile {
            file_name: filename.to_owned(),
            lines: buf.lines().map(|text| Line::new(text.to_owned())).collect(),
        })
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

    pub fn remove(&self, aliases: &[&str]) -> Result<(), ErrorWrapper> {
        alias::validate_aliases(aliases)?;
        // let aliases: HashSet<&&str> = HashSet::from_iter(aliases.iter());
        let mut writer = fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(&self.file_name)
            .map(BufWriter::new)
            .map_err(|error| ErrorWrapper::new_open_error(&self.file_name, error))?;
        let retained_lines = (&self.lines).iter().filter(|line| match line.alias() {
            Some(Ok(alias)) => !aliases.contains(&alias),
            _ => true,
        });
        for line in retained_lines {
            writer
                .write_all(format!("{}\n", line.text()).as_bytes())
                .map_err(|error| ErrorWrapper::new_write_error(&self.file_name, error))?;
        }
        Ok(())
    }

    pub fn is_valid(&self) -> Result<(), ErrorWrapper> {
        for alias in self.all_aliases() {
            if let Err(invalid_alias) = alias {
                return Err(ErrorWrapper::new_invalid_alias_file_error(
                    &self.file_name,
                    invalid_alias,
                ));
            }
        }
        Ok(())
    }

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

    use super::AliasesFile;

    const TEST_FILE: &str = "data/test-aliases";
    const APPEND_TEST_FILE: &str = "data/append-aliases";
    const REMOVE_TEST_FILE: &str = "data/remove-aliases";

    #[test]
    fn from_file_loads_expected_content() {
        for n in 0..5 {
            create_test_file(TEST_FILE, n);
            let aliases_file = AliasesFile::from_file(TEST_FILE).unwrap();
            assert_eq!(aliases_file.lines().len(), n);
            for i in 0..n {
                assert_eq!(aliases_file.lines()[i].text(), format!("a{}.local", i));
            }
            delete_test_file(TEST_FILE);
        }
    }

    #[test]
    fn append_appends() {
        for n in 0..5 {
            create_test_file(APPEND_TEST_FILE, n);
            let aliases_file = AliasesFile::from_file(APPEND_TEST_FILE).unwrap();
            aliases_file
                .append(&vec!["b0.local"])
                .unwrap_or_else(|error| panic!("Append failed: {}", error));
            let aliases_file = AliasesFile::from_file(APPEND_TEST_FILE).unwrap();
            let aliases = aliases_file.aliases();
            assert_eq!(aliases[n], "b0.local");
            delete_test_file(APPEND_TEST_FILE);
        }
    }

    #[test]
    fn remove_removes() -> Result<(), Error> {
        for n in 0..5 {
            create_test_file(REMOVE_TEST_FILE, n);
            let aliases_file = AliasesFile::from_file(REMOVE_TEST_FILE).unwrap();
            if n >= 2 {
                aliases_file
                    .remove(&["a0.local", "a2.local"])
                    .unwrap_or_else(|error| panic!("Remove failed: {}", error));
            } else if n > 0 {
                aliases_file
                    .remove(&["a0.local"])
                    .unwrap_or_else(|error| panic!("Remove failed: {}", error));
            }
            let aliases_file = AliasesFile::from_file(REMOVE_TEST_FILE).unwrap();
            let aliases = aliases_file.aliases();
            if n > 3 {
                assert_eq!(aliases[0], "a1.local");
                assert_eq!(aliases[1], "a3.local");
            } else if n > 1 {
                assert_eq!(aliases[0], "a1.local");
            }
            delete_test_file(REMOVE_TEST_FILE);
        }
        Ok(())
    }

    fn create_test_file(file_name: &str, line_count: usize) {
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)
            .map(BufWriter::new)
            .and_then(|mut writer| {
                for i in 0..line_count {
                    writer.write_all(format!("a{}.local\n", i).as_bytes())?;
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
    }

    fn delete_test_file(file_name: &str) {
        fs::remove_file(file_name).unwrap_or_else(|_| {
            panic!(
                "Could not delete test file: cwd={:?}, file={:?}",
                std::env::current_dir(),
                file_name
            )
        });
    }
}

// end
