use crate::node::NodeType;

use super::Grammar;

pub fn parse_grammar(content: &str) -> Grammar {
    let mut rules: Vec<(NodeType, usize)> = vec![];

    for (i, line) in content.trim().lines().enumerate() {
        let Some((lhs, rhs)) = line.split_once(":") else {
            eprintln!(
                "[WARNING]: Given grammar missing delimeter \":\" at line {}:\n\"{}\"\nIgnoring line.",
                i, line,
            );
            continue;
        };

        let Ok(node_type) = NodeType::try_from(lhs.trim()) else {
            eprintln!(
                "[WARNING]: Given grammar includes not recognized label \"{}\" at line: {}:\n\"{}\"\nIgnoring line.",
                lhs, i, line
            );
            continue;
        };

        let Ok(weight) = rhs.trim().parse::<usize>() else {
            eprintln!(
                "[WARNING]: Given grammar includes invalid weight of \"{}\" at line: {}:\n\"{}\"\nIgnoring line.",
                rhs, i, line
            );
            continue;
        };

        rules.push((node_type, weight));
    }

    Grammar::new(rules)
}
