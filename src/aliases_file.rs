#![warn(clippy::all)]

use std::collections::HashSet;
use std::io::{self, BufWriter, Read, Write};
use std::{self, fs, str};

use super::line::Line;

#[derive(Debug)]
pub struct AliasesFile<'a> {
    file_name: &'a str,
    lines: Vec<Line>,
}

impl<'a> AliasesFile<'a> {
    pub fn aliases(&self) -> Vec<&str> {
        self.lines.iter().filter_map(|line| line.alias()).collect()
    }

    pub fn from_file(filename: &'a str) -> Result<Self, io::Error> {
        fs::OpenOptions::new()
            .read(true)
            .open(filename)
            .and_then(|mut f| {
                let mut buf = String::new();
                match f.read_to_string(&mut buf) {
                    Ok(_) => Ok(buf),
                    Err(e) => Err(e),
                }
            })
            .map(|buffer| {
                let lines = buffer.lines().map(|text| Line::new(text.to_owned()));
                AliasesFile { file_name: filename, lines: lines.collect() }
            })
    }

    pub fn append(&self, aliases: Vec<&str>) -> Result<(), io::Error> {
        fs::OpenOptions::new().append(true).open(self.file_name).map(BufWriter::new).and_then(
            |mut writer| {
                for alias in aliases {
                    writer.write_all(format!("{}\n", alias).as_bytes())?;
                }
                Ok(())
            },
        )
    }

    pub fn remove(&self, aliases: Vec<&str>) -> Result<(), io::Error> {
        let aliases: HashSet<&str> = aliases.into_iter().collect();
        fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(self.file_name)
            .map(BufWriter::new)
            .and_then(|mut writer| {
                let retained_lines = (&self.lines).iter().filter(|l| match l.alias() {
                    Some(alias) => !aliases.contains(alias),
                    _ => true,
                });
                for line in retained_lines {
                    writer.write_all(format!("{}\n", line.text()).as_bytes())?;
                }
                Ok(())
            })
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
    fn append_appends() -> Result<(), Error> {
        for n in 0..5 {
            create_test_file(APPEND_TEST_FILE, n);
            let aliases_file = AliasesFile::from_file(APPEND_TEST_FILE).unwrap();
            aliases_file.append(vec!["b0.local"])?;
            let aliases_file = AliasesFile::from_file(APPEND_TEST_FILE).unwrap();
            let aliases = aliases_file.aliases();
            assert_eq!(aliases[n], "b0.local");
            delete_test_file(APPEND_TEST_FILE);
        }
        Ok(())
    }

    #[test]
    fn remove_removes() -> Result<(), Error> {
        for n in 0..5 {
            create_test_file(REMOVE_TEST_FILE, n);
            let aliases_file = AliasesFile::from_file(REMOVE_TEST_FILE).unwrap();
            if n >= 2 {
                aliases_file.remove(vec!["a0.local", "a2.local"])?;
            } else if n > 0 {
                aliases_file.remove(vec!["a0.local"])?;
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
