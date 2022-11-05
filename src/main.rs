mod RegExp;
mod graph;

use crate::graph::Graph;
use crate::graph::MathGraph;
use RegExp::Regexp;

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
    let mut user_input = String::new();
    let stdin = std::io::stdin(); // We get `Stdin` here.

    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        3 => match (args[1].parse::<String>(), args[2].parse::<String>()) {
            (Ok(text), Ok(regexp)) => {
                println!("Text: {}", text.as_str(),);
                println!("Regexp: {}", regexp.as_str());
                println!(
                    "Result: {}",
                    Regexp::new(regexp.as_str()).match_regexp(text.as_str())
                );
            }
            _ => {
                println!("Invalid arguments");
                return;
            }
        },
        _ => {
            println!("Invalid argument amount");
            return;
        }
    }
}
