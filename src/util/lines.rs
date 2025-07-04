use std::ops::{Add, Index};
use std::slice;

#[derive(Clone, Default, Debug)]
pub struct Line {
    pub number: u64,
    pub text: String,
}

impl Line {
    pub fn new(number: u64, text: String) -> Self {
        Self { number, text }
    }
}

impl Add for Line {
    type Output = Line;

    fn add(self, rhs: Self) -> Self::Output {
        Line {
            number: self.number,
            text: self.text + &rhs.text,
        }
    }
}

pub(crate) struct Lines {
    inner: Vec<Line>,
}

impl Lines {
    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }
}

impl From<String> for Lines {
    fn from(value: String) -> Self {
        let mut lines = vec![];
        let mut line_number = 0;
        for line in value.split('\n') {
            line_number += 1;
            lines.push(Line::new(line_number, String::from(line)));
        }
        Self { inner: lines }
    }
}

impl From<&str> for Lines {
    fn from(value: &str) -> Self {
        let value = value.to_string();
        Lines::from(value)
    }
}

impl<'a> IntoIterator for &'a Lines {
    type Item = &'a Line;
    type IntoIter = slice::Iter<'a, Line>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl Index<i64> for Lines {
    type Output = Line;
    fn index(&self, index: i64) -> &Self::Output {
        let len = self.inner.len();
        let index = if index < 0 {
            (len as i64 + index) as usize
        } else {
            index as usize
        };
        &self.inner[index]
    }
}

mod test {
    use super::*;

    #[test]
    fn split_empty_string() {
        let string = String::from("");
        let lines: Lines = string.into();

        assert_eq!(lines.inner.len(), 1);
        assert_eq!(lines.inner.first().unwrap().number, 1);
        assert_eq!(lines.inner.first().unwrap().text, "");
    }

    #[test]
    fn split_single_line_string() {
        let string = String::from("a");
        let lines: Lines = string.into();
        assert_eq!(lines.inner.len(), 1);
        assert_eq!(lines.inner.first().unwrap().number, 1);
        assert_eq!(lines.inner.first().unwrap().text, "a");
    }

    #[test]
    fn split_multiple_line_string() {
        let string = String::from("a\nb");
        let lines: Lines = string.into();
        assert_eq!(lines.inner.len(), 2);
        assert_eq!(lines.inner.first().unwrap().number, 1);
        assert_eq!(lines.inner.first().unwrap().text, "a");
        assert_eq!(lines.inner[1].number, 2);
        assert_eq!(lines.inner[1].text, "b");
    }
}
