extern crate reachability_solver;

use reachability_solver::solve;

fn main() {
    // This example shows that no initial nodes can reach each other.
    //
    // It means that `(A -> B) ∧ (B -> A)` is alway false,
    // either because they are forming a loop, or because they are disconnected.
    //
    // 1     3
    // |     |
    // 4--5--6
    // |     |
    // 7     9
    // |     |
    // 10-11-12
    //
    println!("{:?}", solve(vec![
        [1, 4], [3, 6],
        [5, 6], [6, 9], [9, 12], [12, 11], [11, 10], [10, 7], [7, 4], [4, 5]
    ]))
}
