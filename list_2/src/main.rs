#[macro_use]
mod graph;

use crate::graph::{Graph, Link};
use std::collections::HashSet;
use maplit::hashmap;

fn main() {
    let graph = Graph::new(
        hashmap!{
            1 => link![(3,10)],
            2 => link![(1,2), (4,1)],
            3 => link![(2,10), (4,5)],
            4 => link![],
        }
    );

    dbg!(graph);
}
