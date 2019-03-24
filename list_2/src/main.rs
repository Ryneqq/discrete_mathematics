#[macro_use]
mod graph;

use crate::graph::{Graph, Link};
use std::collections::HashMap;
use maplit::hashmap;

fn main() {
    // let graph = Graph::new(
    //     hashmap!{
    //         1 => link![(3,10), (2,2)],
    //         2 => link![(3,2), (4,8)],
    //         3 => link![(4,5)],
    //     }
    // );

    let graph = Graph::new(
        hashmap!{
            1 => link![(3,10), (2,2)],
            2 => link![(3,2), (4,8)],
            3 => link![(4,5)],
        }
    );

    dbg!(&graph);
    bellman_ford(1, &graph);
}

fn bellman_ford(source: u32, graph: &Graph) {
    let mut distances = graph.nodes.keys()
        .map(|node| (*node, i32::max_value() as u32))
        .collect::<HashMap<_, _>>();

    let mut predecessors = graph.nodes.iter()
        .map(|(node, _)| (*node, None))
        .collect::<HashMap<u32, Option<u32>>>();

    distances.entry(source)
        .and_modify(|dist| *dist = 0);

    for i in 0..graph.nodes.len() {
        graph.nodes.iter()
            .for_each(|(node, links)| {
                links.iter()
                    .for_each(|link| {
                        let weight  = link.cost;
                        let link    = link.node;

                        if distances[&node] + weight < distances[&link] {
                            let actual_weight = distances[&node] + weight;
                            distances.entry(link)
                                .and_modify(|dist| *dist = actual_weight);
                            predecessors.entry(link)
                                .and_modify(|predecessor| *predecessor = Some(*node));
                        }
                    })
            })
    }

    dbg!(distances);
    dbg!(predecessors);
}
