//! # Map - Converting graph to map and text visualization

/// Stores the content of a cell in the map.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Cell {
    /// Represents a node.
    ///
    /// This means that there is at least one incoming or outgoing edge.
    Node,
    /// Empty cell.
    Empty,
    /// An edge pointing right.
    Right,
    /// An edge pointing left.
    Left,
    /// An edge pointing up.
    Up,
    /// An edge pointing down.
    Down,
    /// A right left bidirectional edge.
    RightLeft,
    /// An up down bidirectional edge.
    UpDown,
    /// A right up edge.
    RightUp,
    /// A right down edge.
    RightDown,
    /// A left up edge.
    LeftUp,
    /// A left down edge.
    LeftDown,
    /// A rising diagonal, reading from left to right.
    DiagRise,
    /// A falling diagonal, reading from left to right.
    DiagFall,
    /// Cross of bidirectional edges.
    Cross,
    /// ```text
    ///     ^
    /// \ /
    ///  X
    /// / \
    ///    v
    /// ```
    CrossRight,
    /// ```text
    /// ^
    /// \ /
    ///  X
    /// / \
    /// v
    /// ```
    CrossLeft,
    /// ```text
    /// ^  ^
    /// \ /
    ///  X
    /// / \
    /// ```
    CrossUp,
    /// ```text
    /// \ /
    ///  X
    /// / \
    /// v  v
    /// ```
    CrossDown,
    /// ```text
    /// ^
    /// \ /
    ///  X
    /// / \
    /// ```
    CrossLeftUp,
    /// ```text
    /// \ /
    ///  X
    /// / \
    /// v
    /// ```
    CrossLeftDown,
    /// ```text
    ///    ^
    /// \ /
    ///  X
    /// / \
    ///    v
    /// ```
    CrossRightUp,
    /// ```text
    /// \ /
    ///  X
    /// / \
    ///    v
    /// ```
    CrossRightDown,
}

impl Cell {
    /// Joins edges with another cell.
    pub fn join(&mut self, other: Cell) {
        match (*self, other) {
            (Cell::Empty, x) => *self = x,

            (Cell::Left, Cell::Right) |
            (Cell::Right, Cell::Left) => *self = Cell::RightLeft,
            (_, Cell::Left) | (_, Cell::Right) |
            (Cell::Left, _) | (Cell::Right, _) => {}

            (Cell::Up, Cell::Down) |
            (Cell::Down, Cell::Up) => *self = Cell::UpDown,
            (_, Cell::Up) | (_, Cell::Down) |
            (Cell::Up, _) | (Cell::Down, _) => {}

            (Cell::LeftUp, Cell::RightDown) |
            (Cell::RightDown, Cell::LeftUp) => *self = Cell::DiagFall,
            (Cell::LeftDown, Cell::RightUp) |
            (Cell::RightUp, Cell::LeftDown) => *self = Cell::DiagRise,

            (Cell::RightUp, Cell::RightDown) |
            (Cell::RightDown, Cell::RightUp) => *self = Cell::CrossRight,
            (Cell::LeftUp, Cell::LeftDown) |
            (Cell::LeftDown, Cell::LeftUp) => *self = Cell::CrossLeft,
            (Cell::RightUp, Cell::LeftUp) |
            (Cell::LeftUp, Cell::RightUp) => *self = Cell::CrossUp,
            (Cell::LeftDown, Cell::RightDown) |
            (Cell::RightDown, Cell::LeftDown) => *self = Cell::CrossDown,

            (Cell::DiagRise, Cell::RightDown) => *self = Cell::CrossRightDown,
            (Cell::DiagRise, Cell::LeftUp) => *self = Cell::CrossLeftUp,
            (Cell::DiagRise, _) => {}
            (Cell::DiagFall, Cell::LeftDown) => *self = Cell::CrossLeftDown,
            (Cell::DiagFall, Cell::RightUp) => *self = Cell::CrossRightUp,
            (Cell::DiagFall, _) => {}

            (Cell::CrossRight, Cell::LeftDown) => *self = Cell::CrossRightDown,
            (Cell::CrossRight, Cell::LeftUp) => *self = Cell::CrossRightUp,
            (Cell::CrossRight, _) => {}
            (Cell::CrossLeft, Cell::RightUp) => *self = Cell::CrossLeftUp,
            (Cell::CrossLeft, Cell::RightDown) => *self = Cell::CrossLeftDown,
            (Cell::CrossLeft, _) => {}
            (Cell::CrossUp, Cell::RightDown) => *self = Cell::CrossRightUp,
            (Cell::CrossUp, Cell::LeftDown) => *self = Cell::CrossLeftUp,
            (Cell::CrossUp, _) => {}
            (Cell::CrossDown, Cell::LeftUp) => *self = Cell::CrossLeftDown,
            (Cell::CrossDown, Cell::RightUp) => *self = Cell::CrossRightDown,
            (Cell::CrossDown, _) => {}

            (Cell::CrossRightUp, Cell::LeftDown) => *self = Cell::Cross,
            (Cell::CrossRightUp, _) => {}
            (Cell::CrossRightDown, Cell::LeftUp) => *self = Cell::Cross,
            (Cell::CrossRightDown, _) => {}
            (Cell::CrossLeftUp, Cell::RightDown) => *self = Cell::Cross,
            (Cell::CrossLeftUp, _) => {}
            (Cell::CrossLeftDown, Cell::RightUp) => *self = Cell::Cross,
            (Cell::CrossLeftDown, _) => {}

            (_, _) => {}
        }
    }

    /// Returns `true` if edges points along a diagonal direction.
    ///
    /// The direction of the diagonal is ignored.
    ///
    /// The valid inputs for diagonal are `[1, 1], [-1, -1], [-1, 1], [1, -1]`.
    pub fn points_along_diag(self, dir: [i8; 2]) -> bool {
        match dir {
            [1, 1] | [-1, -1] => match self {
                Cell::RightDown |
                Cell::LeftUp |
                Cell::DiagFall |
                Cell::Cross |
                Cell::CrossRight |
                Cell::CrossLeft |
                Cell::CrossUp |
                Cell::CrossDown |
                Cell::CrossRightUp |
                Cell::CrossRightDown |
                Cell::CrossLeftDown |
                Cell::CrossLeftUp => true,
                Cell::Empty |
                Cell::Node |
                Cell::Right |
                Cell::Left |
                Cell::Up |
                Cell::Down |
                Cell::RightLeft |
                Cell::UpDown |
                Cell::RightUp |
                Cell::LeftDown |
                Cell::DiagRise => false
            }
            [-1, 1] | [1, -1] => match self {
                Cell::LeftDown |
                Cell::RightUp |
                Cell::DiagRise |
                Cell::Cross |
                Cell::CrossRight |
                Cell::CrossLeft |
                Cell::CrossDown |
                Cell::CrossLeftDown |
                Cell::CrossRightDown |
                Cell::CrossUp |
                Cell::CrossLeftUp |
                Cell::CrossRightUp => true,
                Cell::Empty |
                Cell::Node |
                Cell::Left |
                Cell::Right |
                Cell::Up |
                Cell::Down |
                Cell::RightLeft |
                Cell::UpDown |
                Cell::RightDown |
                Cell::LeftUp |
                Cell::DiagFall => false
            }
            _ => false
        }
    }
}

/// Creates a 2D map.
///
/// This representation assumes that the graph can be represented
/// on a 2D grid. Every cell is connected to neighbor cells only.
///
/// A cell can have up to 8 neighbors, with diagonal edges.
///
/// ```text
/// 0 1 2
/// 3   5
/// 6 7 8
/// ```
///
/// To simplify rendering of the map,
/// both nodes and edges in the graph is projected into cells.
///
/// This means that there is one cell variant that e.g. points left,
/// another that represents a node, etc.
pub fn map2(dim: [usize; 2], x: &[[usize; 2]]) -> Vec<Vec<Cell>> {
    let mut map = vec![vec![Cell::Empty; dim[0] * 2 - 1]; dim[1] * 2 - 1];
    for &[a, b] in x {
        let pa = [a % dim[0], a / dim[0]];
        let pb = [b % dim[0], b / dim[0]];
        let d = [pb[0] as isize - pa[0] as isize, pb[1] as isize - pa[1] as isize];
        match d {
            [1, 0] => map[pa[1] * 2][pa[0] * 2 + 1].join(Cell::Right),
            [-1, 0] => map[pa[1] * 2][pa[0] * 2 - 1].join(Cell::Left),
            [0, 1] => map[pa[1] * 2 + 1][pa[0] * 2].join(Cell::Down),
            [0, -1] => map[pa[1] * 2 - 1][pa[0] * 2].join(Cell::Up),
            [1, 1] => map[pa[1] * 2 + 1][pa[0] * 2 + 1].join(Cell::RightDown),
            [1, -1] => map[pa[1] * 2 - 1][pa[0] * 2 + 1].join(Cell::RightUp),
            [-1, -1] => map[pa[1] * 2 - 1][pa[0] * 2 - 1].join(Cell::LeftUp),
            [-1, 1] => map[pa[1] * 2 + 1][pa[0] * 2 - 1].join(Cell::LeftDown),
            _ => {}
        }
    }

    let h = map.len();
    let w = map[0].len();
    for j in 0..h/2+1 {
        let j = j * 2;
        for i in 0..w {
            map[j][i] = match map[j][i] {
                Cell::Empty => {
                    let left = if i == 0 {false} else {map[j][i-1] != Cell::Empty};
                    let right = if i + 1 == w {false} else {map[j][i+1] != Cell::Empty};
                    let up = if j == 0 {false} else {map[j-1][i] != Cell::Empty};
                    let down = if j + 1 == h {false} else {map[j+1][i] != Cell::Empty};
                    let right_down = if i + 1 == w || j + 1 == h {false}
                                     else {map[j+1][i+1].points_along_diag([1, 1])};
                    let left_up = if i == 0 || j == 0 {false}
                                  else {map[j-1][i-1].points_along_diag([1, 1])};
                    let left_down = if i == 0 || j + 1 == h {false}
                                    else {map[j+1][i-1].points_along_diag([-1, 1])};
                    let right_up = if i + 1 == w || j == 0 {false}
                                   else {map[j-1][i+1].points_along_diag([-1, 1])};
                    let any = left || right || up || down ||
                              right_down || left_up || right_up || left_down;
                    if any && i % 2 == 0 {Cell::Node} else {Cell::Empty}
                },
                _ => continue,
            };
        }
    }

    map
}

/// Generates a string that visualizes the map with unicode symbols.
///
/// Here is an example of a map visualized:
///
/// ```text
/// ■ → ■ → ■
/// ↓   ↓   ↓
/// ■ → ■ → ■
/// ↓   ↓   ↓
/// ■ → ■ → ■
/// ```
///
/// Crossing arrows are shown with unicode symbols, whenever possible:
///
/// ```text
/// ■   ■
///   ⤯
/// ■   ■
/// ```
///
/// Due to shortage of some unicode crossing variants, white arrows are used instead:
///
/// ```text
/// ■   ■
///   ⬃
/// ■   ■
/// ```
///
/// A white arrow means that it crosses a bidirectional edge.
pub fn visualize(map: &Vec<Vec<Cell>>) -> String {
    let mut s = String::new();
    let h = map.len();
    let w = map[0].len();
    for j in 0..h {
        for i in 0..w {
            s.push(match map[j][i] {
                Cell::Right => '→',
                Cell::Left => '←',
                Cell::Down => '↓',
                Cell::Up => '↑',
                Cell::RightLeft => '-',
                Cell::UpDown => '|',
                Cell::RightDown => {s.push_str("↘︎ "); continue},
                Cell::RightUp => {s.push_str("↗︎ "); continue},
                Cell::LeftUp => {s.push_str("↖︎ "); continue},
                Cell::LeftDown => {s.push_str("↙︎ "); continue},
                Cell::DiagRise => '⟋',
                Cell::DiagFall => '⟍',
                Cell::Cross => '╳',
                Cell::CrossRightUp => '⤯',
                Cell::CrossRightDown => '⤰',
                Cell::CrossRight => '⤭',
                Cell::CrossUp => '⤲',
                Cell::CrossLeft => '⤪',
                Cell::CrossDown => '⤩',
                Cell::CrossLeftUp => '⬁',
                Cell::CrossLeftDown => '⬃',
                Cell::Node => '■',
                Cell::Empty => ' ',
            });
            if i + 1 != w {s.push(' ')};
        }
        if j + 1 != h {s.push('\n')};
    }
    s
}
