use clap::Parser;
use grammar::Grammar;
use node::{NodeType, generate};

mod cli;
pub mod grammar;
mod img;
pub mod node;

fn main() {
    let args = cli::Args::parse();

    if args.dump_default_grammar {
        print!("# DEFAULT GRAMMAR\n\n{}", Grammar::default());
        std::process::exit(0);
    }

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
