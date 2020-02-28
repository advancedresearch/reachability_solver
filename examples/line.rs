extern crate reachability_solver;

use reachability_solver::{solve, etch, shapes};

fn main() {
    let n = 4;
    // 0 1 2 3 4
    let mut x = shapes::dir_line(n);
    while x.len() > 0 {
        let m = solve(x.clone());
        println!("{:?}\n---------------------------------------\n{:?}\n", x, m);
        etch::initial(&m, &mut x);
    }
}
