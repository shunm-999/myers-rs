use crate::util::bp_vec::BpVec;
use crate::util::lines::{Line, Lines};

pub struct TextDiff {
    edits: Vec<EditTag>,
}

impl TextDiff {
    pub fn from_lines(old: &str, new: &str) -> Self {
        let old: Lines = old.into();
        let new: Lines = new.into();

        let edits = {
            let edits = TextDiffSolver::diff(&old, &new).unwrap_or(Vec::new());
            let edits: Vec<EditTag> = edits.into_iter().rev().collect();
            let mut edits = edits.compress();
            if edits.is_empty() {
                edits.push(EditTag::Equal {
                    old: Line::new(0, "".to_string()),
                    new: Line::new(0, "".to_string()),
                })
            }
            edits
        };
        Self { edits }
    }
}

struct TextDiffSolver {}

enum TextDiffSolverError {
    NoSolutionFound,
}

#[derive(Clone, Debug)]
pub enum EditTag {
    Insert { new: Line },
    Delete { old: Line },
    Equal { old: Line, new: Line },
}

impl TextDiffSolver {
    fn diff(old: &Lines, new: &Lines) -> Result<Vec<EditTag>, TextDiffSolverError> {
        let trace = Self::search_process(&old, &new)?;
        Ok(Self::back_track(&old, &new, &trace))
    }
    fn search_process(
        old: &Lines,
        new: &Lines,
    ) -> Result<Vec<(BpVec<i64>, i64)>, TextDiffSolverError> {
        let n = old.len();
        let m = new.len();
        let max = n + m;

        let mut v = BpVec::new_with_capacity(2 * max + 1, 0i64);

        let mut trace = vec![];

        for d in 0..=max {
            let d = d as i64;
            trace.push((v.clone(), d));

            for k in ((-d)..=d).step_by(2) {
                let mut x;
                let mut y;

                if k == -d || (k != d && v[k - 1] < v[k + 1]) {
                    x = v[k + 1];
                } else {
                    x = v[k - 1] + 1;
                }
                y = x - k;
                while x < n as i64 && y < m as i64 && old[x].text == new[y].text {
                    // Skip all matching lines
                    x = x + 1;
                    y = y + 1;
                }
                v[k] = x;
                if x >= n as i64 && y >= m as i64 {
                    return Ok(trace);
                }
            }
        }
        Err(TextDiffSolverError::NoSolutionFound)
    }

    fn back_track(old: &Lines, new: &Lines, track: &Vec<(BpVec<i64>, i64)>) -> Vec<EditTag> {
        let mut x = old.len() as i64;
        let mut y = new.len() as i64;

        let mut edits = vec![];

        for i in (0..track.len()).rev() {
            let (v, d) = &track[i];
            let k = x - y;

            let prev_k = if k == -d || (k != *d && v[k - 1] < v[k + 1]) {
                k + 1
            } else {
                k - 1
            };
            let prev_x = v[prev_k];
            let prev_y = prev_x - prev_k;

            while x > prev_x && y > prev_y {
                x = x - 1;
                y = y - 1;
                edits.push(EditTag::Equal {
                    old: old[x].clone(),
                    new: new[y].clone(),
                });
            }

            if *d > 0i64 {
                if x == prev_x {
                    let line = new[prev_y].clone();
                    if line.text != "" {
                        edits.push(EditTag::Insert { new: line });
                    }
                } else if y == prev_y {
                    let line = old[prev_x].clone();
                    if line.text != "" {
                        edits.push(EditTag::Delete { old: line });
                    }
                }
            }
            x = prev_x;
            y = prev_y;
        }

        edits
    }
}

trait MergeAdjacent<T: 'static + Clone>: IntoIterator<Item = T> {
    fn merge_adjacent<F>(self, merge: F) -> Vec<T>
    where
        F: Fn(&T, &T) -> Option<T>;
}

impl<T: 'static + Clone> MergeAdjacent<T> for Vec<T> {
    fn merge_adjacent<F>(self, merge: F) -> Vec<T>
    where
        F: Fn(&T, &T) -> Option<T>,
    {
        let mut iter = self.into_iter().peekable();
        let mut result = Vec::new();

        while let Some(mut current) = iter.next() {
            while let Some(next) = iter.peek() {
                if let Some(merged) = merge(&current, next) {
                    current = merged;
                    iter.next(); // skip merged `next`
                } else {
                    break;
                }
            }
            result.push(current);
        }
        result
    }
}

trait Compress {
    fn compress(self) -> Self;
}

impl Compress for Vec<EditTag> {
    fn compress(self) -> Self {
        self.merge_adjacent(|left, right| match (left, right) {
            (EditTag::Insert { new: left_line }, EditTag::Insert { new: right_line }) => {
                if left_line.number == right_line.number {
                    Some(EditTag::Insert {
                        new: left_line.clone() + right_line.clone(),
                    })
                } else {
                    None
                }
            }
            (EditTag::Delete { old: left_line }, EditTag::Delete { old: right_line }) => {
                if left_line.number == right_line.number {
                    Some(EditTag::Delete {
                        old: left_line.clone() + right_line.clone(),
                    })
                } else {
                    None
                }
            }
            (EditTag::Equal { old: left_line, .. }, EditTag::Delete { old: right_line }) => {
                if left_line.number == right_line.number {
                    let old = left_line.clone() + right_line.clone();
                    Some(EditTag::Equal {
                        old: old.clone(),
                        new: old,
                    })
                } else {
                    None
                }
            }
            (_, _) => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 基本的なテストケース
    #[test]
    fn test_empty_strings() {
        let diff = TextDiff::from_lines("", "");
        assert_eq!(diff.edits.len(), 1);
        match &diff.edits[0] {
            EditTag::Equal { old, new } => {
                assert_eq!(old.text, "");
                assert_eq!(new.text, "");
            }
            _ => panic!("Expected Equal"),
        }
    }

    #[test]
    fn test_empty_old_string() {
        let diff = TextDiff::from_lines("", "hello\nworld");
        assert_eq!(diff.edits.len(), 2);
        match &diff.edits[0] {
            EditTag::Insert { new } => {
                assert_eq!(new.text, "hello");
            }
            _ => panic!("Expected Insert"),
        }
        match &diff.edits[1] {
            EditTag::Insert { new } => {
                assert_eq!(new.text, "world");
            }
            _ => panic!("Expected Insert"),
        }
    }

    #[test]
    fn test_empty_new_string() {
        let diff = TextDiff::from_lines("hello\nworld", "");
        assert_eq!(diff.edits.len(), 2);
        match &diff.edits[0] {
            EditTag::Delete { old } => {
                assert_eq!(old.text, "hello");
            }
            _ => panic!("Expected Delete"),
        }
        match &diff.edits[1] {
            EditTag::Delete { old } => {
                assert_eq!(old.text, "world");
            }
            _ => panic!("Expected Delete"),
        }
    }

    #[test]
    fn test_insert_new_string() {
        let diff = TextDiff::from_lines("hello", "hello\nworld");
        assert_eq!(diff.edits.len(), 2);
        match &diff.edits[0] {
            EditTag::Equal { old, new } => {
                assert_eq!(old.text, "hello");
            }
            _ => panic!("Expected Equal"),
        }
        match &diff.edits[1] {
            EditTag::Insert { new } => {
                assert_eq!(new.text, "world");
            }
            _ => panic!("Expected Insert"),
        }
    }
    #[test]
    fn test_modify_and_insert_new_string() {
        let diff = TextDiff::from_lines("Hi!", "hello\nworld");
        assert_eq!(diff.edits.len(), 3);
        match &diff.edits[0] {
            EditTag::Delete { old } => {
                assert_eq!(old.text, "Hi!");
            }
            _ => panic!("Expected Delete"),
        }
        match &diff.edits[1] {
            EditTag::Insert { new } => {
                assert_eq!(new.text, "hello");
            }
            _ => panic!("Expected Insert"),
        }
        match &diff.edits[2] {
            EditTag::Insert { new } => {
                assert_eq!(new.text, "world");
            }
            _ => panic!("Expected Insert"),
        }
    }
}
