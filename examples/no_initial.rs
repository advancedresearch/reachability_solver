extern crate reachability_solver;

use reachability_solver::solve;

fn main() {
    // This example shows that even if the node `2` is initial,
    // it must reach a terminal node to be included in the solution.
    //
    // 1--2--3
    // |     |
    // 4     6
    // |     |
    // 7--8--9
    //    |
    //    10
    //
    println!("{:?}", solve(vec![
        [1, 2], [2, 3], [3, 6], [6, 9], [9, 8], [8, 7], [7, 4], [4, 1],
        [8, 10]
    ]))
}
