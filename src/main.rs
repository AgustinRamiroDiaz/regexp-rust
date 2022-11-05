mod graph;
use core::panic;

use crate::graph::Graph;
use crate::graph::MathGraph;

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

struct Regexp {
    graph: Box<dyn Graph<RegExpNode, RexExpEdge>>,
    start_node: RegExpNode,
    end_node: RegExpNode,
}

fn parse(input: &str) -> Vec<RegexpSymbol> {
    let mut glyphs = Vec::new();

    let mut should_escape = false;
    for glyph in input.chars() {
        if should_escape {
            should_escape = false;
            glyphs.push(RegexpSymbol::Letter(glyph));
        } else {
            if glyph == '\\' {
                should_escape = true;
            } else {
                glyphs.push(match glyph {
                    '.' => RegexpSymbol::Dot,
                    _ => RegexpSymbol::Letter(glyph),
                });
            }
        }
    }
    glyphs
}

impl Regexp {
    fn new(input: &str) -> Self {
        let glyphs = parse(input);
        let mut graph = MathGraph::new();

        let start_node = RegExpNode {
            name: "".to_string(), // TODO: don't use magic strings
        };

        let mut current_node = start_node.clone();

        for glyph in glyphs {
            let node = RegExpNode {
                name: format!(
                    "{}{}",
                    current_node.name,
                    match glyph {
                        RegexpSymbol::Letter(letter) => letter.to_string(),
                        RegexpSymbol::Dot => ".".to_string(),
                    }
                ),
            };
            graph.add_node(node.clone());

            graph.add_edge(RexExpEdge {
                from: current_node.clone(),
                symbol: glyph,
                to: node.clone(),
            });

            current_node = node;
        }

        Regexp {
            graph: Box::new(graph),
            start_node,
            end_node: current_node,
        }
    }

    fn match_regexp(&self, input: &str) -> bool {
        let mut current_state = &self.start_node;

        for character in input.chars() {
            let outer_edges = self
                .graph
                .edges()
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
            Regexp::new(r"le\.peke").match_regexp("le.peke.la.peke le peke"),
            true
        );

        assert_eq!(
            Regexp::new(r"le peke").match_regexp("le.peke.la.peke le peke"),
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
