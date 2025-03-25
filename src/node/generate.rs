use crate::grammar::Grammar;

use super::{Node, NodeTree};

pub fn generate_tree(grammar: &mut Grammar, depth: usize) -> NodeTree {
    (
        Node::gen_rand(grammar, depth),
        Node::gen_rand(grammar, depth),
        Node::gen_rand(grammar, depth),
    )
}
