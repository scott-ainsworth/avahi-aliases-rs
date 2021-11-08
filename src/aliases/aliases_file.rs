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
    // pub fn lines(&self) -> &Vec<Line> { &self.lines }
    pub fn aliases(&self) -> Vec<&str> {
        self.lines.iter().filter_map(|line| line.alias()).collect()
    }

    pub fn from_file(filename: &'a str) -> Result<Self, io::Error> {
        load_file_contents(filename).map(|buffer| {
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
}

fn load_file_contents(filename: &str) -> Result<String, io::Error> {
    fs::OpenOptions::new().read(true).open(filename).and_then(|mut f| {
        let mut buf = String::new();
        match f.read_to_string(&mut buf) {
            Ok(_) => Ok(buf),
            Err(e) => Err(e),
        }
    })
}

// end
