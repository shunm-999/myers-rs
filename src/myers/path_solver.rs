use crate::util::bp_vec::BpVec;
use crate::util::lines::Lines;

pub struct PathSolver {}

pub enum PathSolverError {
    NoSolutionFound,
}

pub type PathSolverResult = Result<usize, PathSolverError>;

impl PathSolver {
    fn new() -> Self {
        Self {}
    }
}

impl PathSolver {
    fn solve(old: Lines, new: Lines) -> PathSolverResult {
        let n = old.len();
        let m = new.len();

        let max = n + m;

        let mut v = BpVec::new_with_capacity(2 * max + 1, 0i64);

        for d in 0..=max {
            let d = d as i64;
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
                    return Ok(d as usize);
                }
            }
        }
        Err(PathSolverError::NoSolutionFound)
    }
}
