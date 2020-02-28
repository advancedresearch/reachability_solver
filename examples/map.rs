extern crate reachability_solver;

use reachability_solver::{shapes, map};

fn main() {
    let dim = [3, 3];
    // let shape = shapes::diag_rect(dim);
    let shape = vec![
        [0, 4], [3, 1]
        // [0, 4], [4, 0], [3, 1]
    ];
    let map = map::map2(dim, &shape);
    println!("{}", map::visualize(&map));
}
