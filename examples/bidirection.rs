extern crate reachability_solver;

use reachability_solver::solve;

fn main() {
    // This example shows how to encode a bidirectional maze as a directional one.
    //
    // Each start/end node is represented as two nodes,
    // one initial node and one terminal node.
    //
    // The path between the nodes is a cyclical.
    // This could be simply `[5, 11]` and `[11, 5]`,
    // but demonstrated here to show that a cyclical path might not be trivial.
    //
    // Every start/end node in this encoding reaches itself.
    //
    //  1   3
    //   \ /
    // 4--5--6
    // |     |
    // 7     9
    // |     |
    // 10-11-12
    //   /  \
    // 13   15
    //
    println!("{:?}", solve(vec![
        [1, 5], [5, 3],
        [5, 6], [6, 9], [9, 12], [12, 11], [11, 10], [10, 7], [7, 4], [4, 5],
        [13, 11], [11, 15]
    ]))
}
