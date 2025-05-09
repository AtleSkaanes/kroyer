pub mod parse;

use crate::grammar::Grammar;

use super::{Node, NodePtr};

pub struct NodeAst {
    pub r: NodePtr,
    pub g: NodePtr,
    pub b: NodePtr,
}

impl NodeAst {
    pub fn from_grammar(grammar: &mut Grammar, depth: usize) -> Self {
        Self {
            r: Node::gen_rand(grammar, depth),
            g: Node::gen_rand(grammar, depth),
            b: Node::gen_rand(grammar, depth),
        }
    }
}
