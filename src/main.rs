use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
enum RegexpSymbol {
    Letter(char),
    Dot,
}

// TODO: remove copy and use references
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct RegExpNode {
    name: String,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct RexExpEdge {
    from: RegExpNode,
    symbol: RegexpSymbol,
    to: RegExpNode,
}

struct Graph<Node, Edge> {
    nodes: HashSet<Node>,
    edges: HashSet<Edge>,
}

impl<Node, Edge> Graph<Node, Edge>
where
    Node: Eq + std::hash::Hash,
    Edge: Eq + std::hash::Hash,
{
    fn new() -> Graph<Node, Edge> {
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
    graph: Graph<RegExpNode, RexExpEdge>,
    start_node: RegExpNode,
    end_node: RegExpNode,
}

impl Regexp {
    fn new(input: &str) -> Self {
        let mut graph = Graph::new();

        let start_node = RegExpNode {
            name: "".to_string(), // TODO: don't use magic strings
        };

        let mut current_node = start_node.clone();

        for character in input.chars() {
            let node = RegExpNode {
                name: format!("{}{}", current_node.name, character),
            };
            graph.add_node(node.clone());

            graph.add_edge(RexExpEdge {
                from: current_node.clone(),
                symbol: match character {
                    '.' => RegexpSymbol::Dot,
                    _ => RegexpSymbol::Letter(character),
                },
                to: node.clone(),
            });

            current_node = node;
        }

        Regexp {
            graph,
            start_node,
            end_node: current_node,
        }
    }

    fn match_regexp(&self, input: &str) -> bool {
        let mut current_state = &self.start_node;

        for character in input.chars() {
            let outer_edges = self
                .graph
                .edges
                .iter()
                .filter(|n| n.from == *current_state)
                .collect::<Vec<&RexExpEdge>>();

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

        *current_state == self.end_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Regexp::new("le peke").match_regexp("le.peke.la.peke le peke"),
            true
        );

        assert_eq!(Regexp::new("abcde").match_regexp("123abcd123abcde"), true);

        assert_eq!(Regexp::new("a.c").match_regexp("123abc123"), true);

        assert_eq!(
            Regexp::new("le peke").match_regexp("le.peke.la.peke le-peke"),
            false
        );

        assert_eq!(
            Regexp::new("le.peke").match_regexp("le.peke.la.peke le-peke"),
            true
        );
    }
}

fn main() {
    println!("Hello, world!");
}
