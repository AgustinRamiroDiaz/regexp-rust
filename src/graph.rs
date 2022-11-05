use std::collections::HashSet;

pub(super) struct MathGraph<Node, Edge> {
    pub(super) nodes: HashSet<Node>,
    pub(super) edges: HashSet<Edge>,
}

impl<Node, Edge> MathGraph<Node, Edge>
where
    Node: Eq + std::hash::Hash,
    Edge: Eq + std::hash::Hash,
{
    pub(super) fn new() -> MathGraph<Node, Edge> {
        MathGraph {
            nodes: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node);
    }

    fn add_edge(&mut self, edge: Edge) {
        self.edges.insert(edge);
    }

    fn get_edges(&self) -> &HashSet<Edge> {
        &self.edges
    }
}

pub(super) trait Graph<Node, Edge> {
    fn add_node(&mut self, node: Node);

    fn add_edge(&mut self, edge: Edge);

    fn edges(&self) -> &HashSet<Edge>;
}

impl<Node, Edge> Graph<Node, Edge> for MathGraph<Node, Edge>
where
    Node: Eq + std::hash::Hash,
    Edge: Eq + std::hash::Hash,
{
    fn add_node(&mut self, node: Node) {
        MathGraph::add_node(self, node)
    }

    fn add_edge(&mut self, edge: Edge) {
        MathGraph::add_edge(self, edge)
    }

    fn edges(&self) -> &HashSet<Edge> {
        MathGraph::get_edges(self)
    }
}
