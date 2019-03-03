use std::collections::BTreeMap;

pub struct Graph {
    nodes: Vec<Node>
}

#[derive(Default)]
struct  Node {
    pub links: BtreeMap<String, Vec<Link>>
}

impl Node {
    pub AddLink<T: Into<String>>(&mut self, key: T, value: Link) {
        self.links
            .entry(key.into())
            .and_modify(|e| e.push(value))
            .or_insert(vec![value]);
    }
}

struct Link {
    pub node: String,
    pub cost: usize
}

impl<'a> Link<'a> {
    pub fn new(node: String, cost: usize) -> Self {
        Self {
            node, cost
        }
    }
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        Self {
            nodes: vec![]
        }
    }
}

impl<'a, T> From<&[(T, usize)]> for Graph<'a>
where T: PartialEq {
    fn from(from: &'a [(T, usize)]) -> Graph {
        let links = from.len();
        let mut nodes = from.iter()
            .map(|(node, _)| node)
            .collect::<Vec<_>>();
        nodes.dedup();

        let mut graph = vec![];
        (0..nodes.len()).for_each(|_| graph.push(Node::default()));

        let mut from = from.into::<Vec<_>>();
        from.sort();

        let mut i = 0;
        graph.iter_mut()
            .map(|mut node| {
                from.iter()
                    .map(|(link, cost)| node.AddLink(link, cost))
                    // Logical error, i cannot guess to which node this is linked to or other way arond


                // let name = from.first()
                //     .map(|(n, _)| n)
                //     .unwrap();

                // from.iter()
                //     .take_while(|(n, _)| name == n)
                //     .for_each(|(_, c)| node.links.push(Link::new(&graph[i], c)));

                // from = from.into_iter()
                //     .skip_while(|(n, _)| name == n)
                //     .collect();

                // i += 1;
            }).for_each(|_| ());
        
        Graph::new()
    }
} 