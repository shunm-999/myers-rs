use crate::util::bp_vec::BpVec;
use crate::util::lines::{Line, Lines};

pub struct TextDiff {
    edits: Vec<EditTag>,
}

impl TextDiff {
    pub fn from_lines(old: &str, new: &str) -> Self {
        let old: Lines = old.into();
        let new: Lines = new.into();

        let edits = TextDiffSolver::diff(&old, &new).unwrap_or(Vec::new());
        let edits = edits.compress();
        Self { edits }
    }
}

struct TextDiffSolver {}

enum TextDiffSolverError {
    NoSolutionFound,
}

#[derive(Clone)]
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

        for i in (0..=track.len()).rev() {
            let (v, d) = &track[i];
            let k = x - y;

            let prev_k = if k == -d || (k != *d && v[k - 1] < v[k + 1]) {
                k + 1
            } else {
                k - 1
            };
            let prev_x = v[prev_k];
            let prev_y = prev_k - prev_x;

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
                    edits.push(EditTag::Insert {
                        new: new[prev_y].clone(),
                    });
                } else if y == prev_y {
                    edits.push(EditTag::Delete {
                        old: old[prev_x].clone(),
                    });
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
