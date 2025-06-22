use std::slice;

struct Line {
    number: u64,
    text: String,
}

impl Line {
    pub fn new(number: u64, text: String) -> Self {
        Self { number, text }
    }
}

struct Lines {
    inner: Vec<Line>,
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

impl<'a> IntoIterator for &'a Lines {
    type Item = &'a Line;
    type IntoIter = slice::Iter<'a, Line>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
