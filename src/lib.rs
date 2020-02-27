//! # reachability_solver
//!
//! A linear reachability solver for directional edges
//!
//! This solver takes a list of pair of numbers and produces a new list of pair of numbers.
//!
//! For example:
//!
//! ```text
//! [1, 2], [2, 3]  =>  [1, 3]
//! ```
//!
//! With other words, it is an algorithm that tells whether a
//! [maze](https://en.wikipedia.org/wiki/Maze) is solvable.
//! It does this by reducing the maze to an equivalent maze that is easier to solve.
//!
//! ### Harry Potter and Higher Order Maze Solving
//!
//! *SPOILER ALERT*
//!
//! Such transformed mazes are interesting because they contain the "same information"
//! as the more complex maze with respect to start and end positions.
//!
//! This transformation can be used to pre-process mazes for higher order maze solving.
//! It is called "pre-language semantics" since any reasoning problem using Boolean algebra
//! on the original maze can be solved using Boolean algebra on transformed maze.
//!
//! This idea is best illustrated using an example from the book
//! [Harry Potter and the Goblet of Fire](https://en.wikipedia.org/wiki/Harry_Potter_and_the_Goblet_of_Fire).
//!
//! The maze in the book is very complex and changes over time (a static maze in space-time, btw),
//! so I will use a much simpler example instead.
//!
//! In the final round of the Triwizard Tournament, Harry starts at `1` and Cedric starts at `2`.
//! Both Harry and Cedric try to reach Triwizard Cup at `5`.
//!  We want to know whether Harry and Cedric can reach The Triwizard Cup at the same time.
//! It turns out that they can, because it is reachable for both Harry and Cedric:
//!
//! ```text
//! [1, 4], [2, 3], [3, 4], [4, 5]  =>  [1, 5], [2, 5]
//! ```
//!
//! The pair `[3, 4]` represents the moment when Harry helps Cedric.
//! In an alternative timeline where Harry does not help Cedric, the transformed maze becomes:
//!
//! ```text
//! [1, 4], [2, 3], [4, 5]  =>  [1, 5], [2, 3]
//! ```
//!
//! So, Cedric gets stuck in a dead end of the maze.
//!
//! This is a higher order maze problem, which can be expressed as following:
//!
//! ```text
//! (Harry -> TriwizardCup) ∧ (Cedric -> TriwizardCup)
//! ```
//!
//! Here, the `∧` operator means logical AND.
//!
//! From any maze problem, one can construct a "higher order maze problem" using
//! Boolean algebra on any sub-problem of the kind `A -> B`.
//! These kinds of problems are harder to solve directly on the non-transformed maze,
//! without knowing the details of the higher order problem.
//!
//! Assume that some manipulative person (I won't tell you who)
//! wants Harry to reach Triwizard Cup, but not Cedric.
//! One can express this goal as:
//!
//! ```text
//! (Harry -> TriwizardCup) ∧ ¬(Cedric -> TriwizardCup)
//! ```
//!
//! It is possible to determine whether this is true from a transformed maze,
//! without knowing the internal configurations of how space-time positions are connected.
//!
//! However, if the person places a magical gate that only lets the first person through in the only
//! path toward the goal, then it is not possible to determine whether Harry will reach Triwizard Cup.
//! One must analyze this problem using the original maze and some extra information,
//! such as which path takes shorter time and how fast each wizard champion runs.
//! This problem is undecidable with respect to the solver algorithm.
//!
//! ### Graph Theory
//!
//! The pair of numbers represents edges in a graph.
//! The output produces a pair for every end node reachable from a start node.
//! All other edges are removed, including circular edges.
//!
//! For more information about reachability, see the
//! [Wikipedia article](https://en.wikipedia.org/wiki/Reachability).
//!
//! This can be used to:
//!
//! - Learn about maze-solving algorithms
//! - Learn about linear solving through studying a simple example
//! - Learn about category theory by studying a simple example
//! - Study pre-language semantics (invariant inference over Boolean algebra interpretations)
//!
//! ### Introduction to Category Theory and Reachability
//!
//! A category can be thought of as a kind of graph/network with nodes and edges.
//!
//! - Nodes in category theory is called "objects"
//! - Edges in category theory are directional and called "morphisms"
//!
//! Category theory is a branch of mathematics that abstracts over
//! problems with similar structure, such that reasoning about these
//! problems can be formulated in a common language.
//!
//! The following requirements holds for all categories:
//!
//! - Every object (node) has at least one morphism (arrow) to itself
//! - If there is a morphism (arrow) `A -> B` and `B -> C`, then
//!   there is a morphism (arrow) `A -> C`
//!
//! There is no information about the nodes and edges themselves,
//! only how the nodes and edges are connected.
//!
//! An edge `[A, B]` can be thought of as a proof that there exists
//! at least one morphism `A -> B`.
//!
//! Every object `A` has at least one morphism `A -> A`.
//! Therefore, the edge `[A, A]` is trivial.
//!
//! In a category there are initial and terminal objects:
//!
//! - An initial object has no incoming morphisms
//! - A terminal object has no outgoing morphisms
//!
//! An edge of type `[<initial>, <terminal>]` describes the reachability
//! from an initial object to a terminal object.

#![deny(missing_docs)]

extern crate linear_solver;

use linear_solver::{solve_minimum, Inference};
use linear_solver::Inference::*;

use std::collections::HashSet;

/// Returns a list of edges that describes reachability
/// from initial objects to terminal objects.
pub fn solve(edges: Vec<[usize; 2]>) -> Vec<[usize; 2]> {
    solve_minimum(edges, infer)
}

fn infer(cache: &HashSet<[usize; 2]>, facts: &[[usize; 2]]) -> Option<Inference<[usize; 2]>> {
    for ea in facts {
        let [a, b] = ea;
        for eb in facts {
            let [c, d] = eb;
            if b == c {
                let p = [*a, *d];
                if !cache.contains(&p) {
                    return Some(Propagate(p));
                }
            }
        }
    }

    let mut r = vec![];
    for ea in facts {
        let [a, b] = ea;
        for eb in facts {
            let [c, d] = eb;
            if c == b || a == d {
                r.push(ea.clone());
            }
        }
    }

    return Some(ManyTrue {from: r});
}
