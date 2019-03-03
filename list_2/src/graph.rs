use std::collections::{HashMap, HashSet};
use maplit::hashset;

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<u32, HashSet<Link>>
}

impl Graph {
    pub fn new(input_nodes: HashMap<u32, Vec<Link>>) -> Self {
        let mut nodes: HashMap<u32, HashSet<Link>> = input_nodes.iter()
            .map(|(node, links)|
                (*node, links.iter().cloned().collect()))
            .collect();

        for (node, links) in input_nodes {
            for link in links {
                nodes.entry(link.node)
                    .and_modify(|links| {
                        links.insert(Link::new(node, link.cost));
                    }).or_insert(hashset!{
                        Link::new(node, link.cost)
                    });
            }
        }

        Self {
            nodes
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Link {
    pub node: u32,
    pub cost: u32
}

impl Link {
    pub fn new(node: u32, cost: u32) -> Self {
        Self {
            node, cost
        }
    }
}

impl From<(u32, u32)> for Link {
    fn from((node, cost): (u32, u32)) -> Link {
        Link::new(node, cost)
    }
}

macro_rules! link {
    ($($x: expr),*) => {{
        let mut vector = Vec::new();
        $(vector.push(Link::from($x));)*
        vector
    }}
}
