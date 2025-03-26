use std::{fmt::Display, fs::OpenOptions, io::Read, path::PathBuf};

use rand::Rng;

use crate::{node::NodeType, rng};

/// Holds the node and the weigth of the node in the tree
#[derive(Clone, Debug)]
pub struct Grammar {
    pub rules: Vec<(NodeType, usize)>,
}

impl Grammar {
    pub fn new(rules: Vec<(NodeType, usize)>) -> Self {
        Self { rules }
    }

    pub fn pick(&mut self) -> NodeType {
        let total = self.rules.iter().fold(0, |a, x| a + x.1);

        if total == 0 {
            return NodeType::Literal;
        }

        let choice = rng::get_rng().random_range(0..total);

        let mut acc = 0;
        for rule in &self.rules {
            acc += rule.1;
            if choice < acc {
                return rule.0;
            }
        }
        panic!("CHOICE SHOULD ALWAYS BE UNDER TOTAL WEIGHTS");
    }

    /// Parses a Grammar struct from a given string.
    /// The grammar of a grammar file is as such:
    /// `node: weight`
    /// E.g.
    /// ```
    /// x: 1
    /// y: 1
    /// sub: 2
    /// add: 3
    /// ```
    pub fn parse_from_str(content: &str) -> Self {
        let mut rules: Vec<(NodeType, usize)> = vec![];

        for (i, line) in content.trim().lines().enumerate() {
            let (rule, _) = line.split_once("#").unwrap_or((line, ""));
            if rule.trim().is_empty() {
                continue;
            }

            let Some((lhs, rhs)) = rule.split_once(":") else {
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

    /// Parses a Grammar struct from a given file, via `Grammar::parse_from_str()`
    pub fn parse_from_file(path: PathBuf) -> Self {
        let mut file = match OpenOptions::new().read(true).open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!(
                    "[ERROR]: Failed to open grammar file {:?}.\nDetails: {}",
                    path, e
                );
                std::process::exit(1);
            }
        };

        let mut buf = String::new();
        if let Err(e) = file.read_to_string(&mut buf) {
            eprintln!(
                "[ERROR]: Failed to read grammar file {:?}.\nDetails: {}",
                path, e
            );
            std::process::exit(1);
        };

        if buf.trim().is_empty() {
            eprintln!(
                "[WARNING]: Given grammar file is empty. Use --dump-default-grammar to get the default grammar file"
            );
        }

        Self::parse_from_str(&buf)
    }
}

impl Default for Grammar {
    fn default() -> Self {
        let rules = vec![
            (NodeType::X, 1),
            (NodeType::Y, 1),
            (NodeType::Literal, 1),
            (NodeType::Mod, 3),
            (NodeType::Sin, 5),
            (NodeType::Tan, 4),
            (NodeType::Mult, 3),
            (NodeType::Add, 3),
            (NodeType::Sqrt, 3),
            (NodeType::Max, 3),
            (NodeType::If, 1),
        ];

        Grammar::new(rules)
    }
}

impl Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (node, weight) in &self.rules {
            writeln!(f, "{}: {}", node, weight)?;
        }
        Ok(())
    }
}
