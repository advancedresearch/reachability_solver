//! ### Shapes - Some functions to create various shapes of mazes.

/// Creates a directional line of length `n`, starting at `0`.
///
/// For example, `dir_line(4)`:
///
/// ```text
/// 0 → 1 → 2 → 3
/// ```
pub fn dir_line(n: usize) -> Vec<[usize; 2]> {
    let mut r = vec![];
    if n == 0 {return r}
    for i in 0..n-1 {
        r.push([i, i+1])
    }
    r
}

/// Creates a rectangle where every cell is connected to its
/// right and down neighbors.
///
/// For example:
///
/// ```text
/// 0 → 1 → 2
/// ↓   ↓   ↓
/// 3 → 4 → 5
/// ↓   ↓   ↓
/// 6 → 7 → 8
/// ```
pub fn diag_rect(n: [usize; 2]) -> Vec<[usize; 2]> {
    let mut r = vec![];
    for j in 0..n[1] {
        for i in 0..n[0] {
            let x = j * n[0] + i;
            if i+1 != n[0] {r.push([x, x+1])}
            if j+1 != n[1] {r.push([x, x+n[0]])}
        }
    }
    r
}
