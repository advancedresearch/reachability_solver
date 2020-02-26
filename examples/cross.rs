extern crate reachability_solver;

use reachability_solver::solve;

fn main() {
    //   2
    // 4 5 6
    //   8
    println!("{:?}", solve(vec![
        [2, 5], [4, 5], [5, 6], [5, 8]
    ]))
}
