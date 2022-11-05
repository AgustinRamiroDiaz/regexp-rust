use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
enum RegexpSymbol {
    Letter(char),
    Dot,
}

// TODO: remove copy and use references
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Node {
    name: String,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Edge {
    from: Node,
    symbol: RegexpSymbol,
    to: Node,
}

struct Graph {
    nodes: HashSet<Node>,
    edges: HashSet<Edge>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
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
}

struct Regexp {
    graph: Graph,
    start_node: Node,
    end_node: Node,
}

impl Regexp {
    fn match_regexp(&self, input: String) -> bool {
        let mut current_state = &self.start_node;

        for character in input.chars() {
            let outer_edges = self
                .graph
                .edges
                .iter()
                .filter(|n| n.from == *current_state)
                .collect::<Vec<&Edge>>();

            for edge in outer_edges {
                if edge.symbol == RegexpSymbol::Letter(character) {
                    current_state = &edge.to;
                } else if edge.symbol == RegexpSymbol::Dot && character != '\n' {
                    current_state = &edge.to;
                } else {
                    current_state = &self.start_node;
                }
            }
        }

        current_state == &self.end_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // let regular_expression = regexp::from("abc");
        // let regular_expression = vec![RegexpSymbol::Letter('a'), RegexpSymbol::Dot, RegexpSymbol::Letter('c')];

        let mut graph = Graph::new();

        let start_node = Node {
            name: "start".to_string(),
        };
        let end_node = Node {
            name: "end".to_string(),
        };
        let a_node = Node {
            name: "a".to_string(),
        };
        let a_dot_node = Node {
            name: "a.".to_string(),
        };

        graph.nodes.insert(start_node.clone());
        graph.nodes.insert(end_node.clone());
        graph.nodes.insert(a_node.clone());
        graph.nodes.insert(a_dot_node.clone());

        graph.edges.insert(Edge {
            from: start_node.clone(),
            symbol: RegexpSymbol::Letter('a'),
            to: a_node.clone(),
        });

        graph.edges.insert(Edge {
            from: a_node.clone(),
            symbol: RegexpSymbol::Dot,
            to: a_dot_node.clone(),
        });

        graph.edges.insert(Edge {
            from: a_dot_node.clone(),
            symbol: RegexpSymbol::Letter('c'),
            to: end_node.clone(),
        });

        let regular_expression = Regexp {
            graph,
            start_node,
            end_node,
        };

        assert_eq!(
            regular_expression.match_regexp("123abc123".to_string()),
            true
        );

        assert_eq!(
            regular_expression.match_regexp("123ac123".to_string()),
            false
        );
    }
}

fn main() {
    println!("Hello, world!");
}
