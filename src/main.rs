use grammar::Grammar;
use node::{NodeType, generate};

mod cli;
pub mod grammar;
mod img;
pub mod node;

fn main() {
    //let mut grammar = Grammar::new(vec![
    //    (NodeType::X, 1),
    //    (NodeType::Y, 1),
    //    (NodeType::T, 3),
    //    (NodeType::Literal, 1),
    //    (NodeType::Rand, 1),
    //    (NodeType::Add, 4),
    //    (NodeType::Sub, 4),
    //    (NodeType::Mult, 4),
    //    (NodeType::Div, 4),
    //    (NodeType::Sqrt, 3),
    //    (NodeType::Pow, 3),
    //    (NodeType::Max, 3),
    //    (NodeType::Min, 3),
    //    (NodeType::Sin, 5),
    //    (NodeType::Tan, 5),
    //    (NodeType::Cos, 5),
    //    (NodeType::Mod, 5),
    //    (NodeType::Abs, 1),
    //    (NodeType::If, 3),
    //]);

    let mut grammar = Grammar::parse_from_file("./test.kroyer");

    let tree = generate::generate_tree(&mut grammar, 5);
    println!("## R:\n{}\n## G:\n{}\n## B:\n{}", tree.0, tree.1, tree.2);

    let has_t = grammar.rules.iter().any(|x| x.0 == NodeType::T);

    if has_t {
        img::gen_gif("test.gif", 512, 512, 100, &tree);
    } else {
        img::gen_img("test.png", 512, 512, &tree);
    }
}
