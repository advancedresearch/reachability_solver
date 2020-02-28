extern crate reachability_solver;

use reachability_solver::{solve, etch, shapes, map};

fn main() {
    let n = 4;
    // 0 1 2 3
    let mut x = shapes::dir_line(n);
    while x.len() > 0 {
        let m = solve(x.clone());
        println!("{}", map::visualize(&map::map2([4, 1], &x)));
        println!("{:?}\n---------------------------------------\n{:?}\n", x, m);
        etch::initial(&m, &mut x);
    }
}
