use crate::util::bp_vec::BpVec;
use crate::util::lines::Lines;

struct PathSolver {}

impl PathSolver {
    fn new() -> Self {
        Self {}
    }
}

impl PathSolver {
    fn solve(old: Lines, new: Lines) -> usize {
        let n = old.len();
        let m = new.len();

        let max = n + m;

        // let v = BpVec::new();
        //
        // for d in 0..=max {
        //     let d = d as i64;
        //     for k in ((-d)..=d).step_by(2) {
        //         if k == -d || (k != d && v[k - 1] < v[k + 1]) {}
        //     }
        // }
        todo!()
    }
}
