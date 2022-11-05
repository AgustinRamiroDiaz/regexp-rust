use crate::graph::MathGraph;

use crate::graph::Graph;

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) enum RegexpSymbol {
    Letter(char),
    Dot,
}

// TODO: remove copy and use references
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub(crate) struct RegExpNode {
    pub(crate) name: String,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) struct RexExpEdge {
    pub(crate) from: RegExpNode,
    pub(crate) symbol: RegexpSymbol,
    pub(crate) to: RegExpNode,
}

pub(crate) struct Regexp {
    pub(crate) graph: Box<dyn Graph<RegExpNode, RexExpEdge>>,
    pub(crate) start_node: RegExpNode,
    pub(crate) end_node: RegExpNode,
}

pub(crate) fn parse(input: &str) -> Vec<RegexpSymbol> {
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
    pub(crate) fn new(input: &str) -> Self {
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

    pub(crate) fn match_regexp(&self, input: &str) -> bool {
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
