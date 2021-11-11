#![warn(clippy::all)]

#[derive(Debug)]
pub struct Line {
    line: String,
}

impl Line {
    pub fn new(line: String) -> Self { Self { line } }
    pub fn alias(&self) -> Option<&str> { clean_alias(&self.line) }
    pub fn text(&self) -> &str { &self.line }
}

fn clean_alias(raw_alias: &str) -> Option<&str> {
    let line_without_comment = match raw_alias.find('#') {
        Some(i) => &raw_alias[0..i],
        None => raw_alias,
    };
    match line_without_comment.trim() {
        empty if empty.is_empty() => None,
        alias => Some(alias),
    }
}

#[cfg(test)]
// Utility macros to simplify tests
#[allow(unused_macros)] // work around compiler bug
macro_rules! option_from_text {
    ( $text:ident ) => {
        Line::new(String::from($text)).alias()
    };
}

#[allow(unused_macros)] // work around compiler bug
macro_rules! alias_from_text {
    ( $text:ident ) => {
        // Line::new(String::from($text)).alias().unwrap()
        option_from_text!($text).unwrap()
    };
}

#[test]
fn alias_only_yields_alias() {
    let data = ["a.local", "xyzzy.local"];
    for text in data {
        assert_eq!(alias_from_text!(text), text);
    }
}

#[test]
fn whitespace_is_ignored() {
    let data = [
        "a.local  ",
        "  a.local",
        "  a.local   ",
        "a.local\t",
        "\ta.local",
        "\ta.local\t",
        "a.local \t ",
        " \t a.local",
        " \t a.local \t ",
    ];
    for text in data {
        assert_eq!(alias_from_text!(text), "a.local");
    }
}

#[test]
fn trailing_comments_are_ignored() {
    let data = [
        "a.local# Comment",
        "a.local # Comment",
        "a.local  # Comment",
        "a.local \t # Comment",
        "a.local # A Long, Long Comment",
    ];
    for text in data {
        assert_eq!(alias_from_text!(text), "a.local");
    }
}

#[test]
fn comments_and_whitespace_are_ignored() {
    let data = [
        "a.local # Comment",
        "a.local #  Comment",
        "a.local # \t Comment",
        "a.local # Comment ",
        "a.local #  Comment\t",
        "a.local # \t Comment \t ",
        " a.local # Comment",
        "\ta.local #  Comment",
        " \t a.local # \t Comment",
        " a.local # Comment ",
        "\ta.local #  Comment\t",
        " \t a.local # \t Comment \t ",
    ];
    for text in data {
        assert_eq!(alias_from_text!(text), "a.local");
    }
}

#[test]
fn whitespace_lines_yield_none() {
    let data = ["", " ", "  ", "\t", "\t", "\t\t", " \t ", " \t ", " \t \t "];
    for text in data {
        assert_eq!(option_from_text!(text), None)
    }
}

#[test]
fn comment_only_lines_yield_none() {
    let data = ["# Comment", " # Comment", " # Comment    "];
    for text in data {
        assert_eq!(option_from_text!(text), None)
    }
}

// end
