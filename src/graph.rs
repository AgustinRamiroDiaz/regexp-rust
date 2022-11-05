use std::collections::HashSet;

pub(super) struct Graph<Node, Edge> {
    pub(super) nodes: HashSet<Node>,
    pub(super) edges: HashSet<Edge>,
}

impl<Node, Edge> Graph<Node, Edge>
where
    Node: Eq + std::hash::Hash,
    Edge: Eq + std::hash::Hash,
{
    pub(super) fn new() -> Graph<Node, Edge> {
        Graph {
            nodes: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    pub(super) fn add_node(&mut self, node: Node) {
        self.nodes.insert(node);
    }

    pub(super) fn add_edge(&mut self, edge: Edge) {
        self.edges.insert(edge);
    }
}
