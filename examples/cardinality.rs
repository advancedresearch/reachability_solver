extern crate reachability_solver;

use reachability_solver::{etch, shapes};

fn main() {
    let shape = shapes::diag_rect([3, 3]);
    println!("{:?}", shape);
    println!("{}", etch::cardinality(&shape));
}
