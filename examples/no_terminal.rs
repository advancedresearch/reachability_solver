extern crate reachability_solver;

use reachability_solver::solve;

fn main() {
    // This example shows that even if the node `2` is initial,
    // it must reach a terminal node to be included in the solution.
    //
    //    2
    //    |
    // 4--5--6
    // |     |
    // 7     9
    // |     |
    // 10-11-12
    //
    println!("{:?}", solve(vec![
        [2, 5],
        [5, 6], [6, 9], [9, 12], [12, 11], [11, 10], [10, 7], [7, 4], [4, 5]
    ]))
}
