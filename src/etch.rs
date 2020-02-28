//! ### Etching - A technique to iterate through a maze by removing nodes
//!
//! Etching is a technique to iterate through a maze by
//! removing either initial or terminal nodes.
//!
//! The maze solution contains information about which nodes that are initial and terminal.
//! This information can be used to change the original maze.
//! Solving the new maze yields a different solution than the previous one.
//!
//! The purpose of etching is to analyze a maze without looking at the original.
//! Various techniques for etching puts restrictions to what kind of knowledge is obtained.

/// Etches away initial nodes.
///
/// The first argument should be the solved maze.
/// The second argument should be the original maze.
pub fn initial(a: &[[usize; 2]], b: &mut Vec<[usize; 2]>) {
    for i in 0..a.len() {
        for j in (0..b.len()).rev() {
            if a[i][0] == b[j][0] {
                b.swap_remove(j);
            }
        }
    }
}

/// Etches away terminal nodes.
pub fn terminal(a: &[[usize; 2]], b: &mut Vec<[usize; 2]>) {
    for i in 0..a.len() {
        for j in (0..b.len()).rev() {
            if a[i][1] == b[j][1] {
                b.swap_remove(j);
            }
        }
    }
}

/// Measures the cardinality of a maze by removing initial nodes repeatedly,
/// until there are no edges left.
///
/// The cardinality is the same whether initial or terminal nodes are removed.
///
/// The cardinality of an empty maze is zero.
///
/// The cardinality measures the maximum number of steps required to
/// reach any goal, plus one.
pub fn cardinality(x: &[[usize; 2]]) -> usize {
    use crate::solve;

    if x.len() == 0 {return 0}
    let mut b: Vec<[usize; 2]> = x.into();
    let mut n = 1;
    while b.len() > 0 {
        let a = solve(b.clone());
        initial(&a, &mut b);
        n += 1;
    }
    n
}
