extern crate reachability_solver;

use reachability_solver::solve;

fn main() {
    // 1 2 3
    // 4   6
    // 7 8 9
    println!("{:?}", solve(vec![
        [1, 2], [2, 3], [1, 4], [3, 6],
        [4, 7], [6, 9], [7, 8], [8, 9]
    ]))
}
