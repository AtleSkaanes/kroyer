use rand::Rng;

use super::{Node, NodePtr, NodeType};

/// Holds the node and the weigth of the node in the tree
pub struct Grammar {
    pub grammar: Vec<(NodeType, usize)>,
    pub rng: rand::prelude::ThreadRng,
}

impl Grammar {
    pub fn new(grammar: Vec<(NodeType, usize)>) -> Self {
        Self {
            grammar,
            rng: rand::rng(),
        }
    }

    pub fn pick(&mut self) -> NodeType {
        let total = self.grammar.iter().fold(0, |a, x| a + x.1);

        let choice = self.rng.random_range(0..total);

        let mut acc = 0;
        for rule in &self.grammar {
            acc += rule.1;
            if choice < acc {
                return rule.0;
            }
        }
        panic!("CHOICE SHOULD ALWAYS BE UNDER TOTAL WEIGHTS");
    }
}

pub fn generate_tree(grammar: &mut Grammar, depth: usize) -> (NodePtr, NodePtr, NodePtr) {
    (
        Node::gen_rand(grammar, depth),
        Node::gen_rand(grammar, depth),
        Node::gen_rand(grammar, depth),
    )
}
