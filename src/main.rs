use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use grammar::Grammar;
use node::{NodeType, generate};
use primitive_types::U256;

mod cli;
pub mod grammar;
mod img;
pub mod io;
pub mod node;
pub mod rng;

fn main() {
    let args = cli::Args::parse();

    // Handle flags that cancel all other operations
    if args.dump_default_grammar {
        print!("# DEFAULT GRAMMAR\n\n{}", Grammar::default());
        std::process::exit(0);
    }

    let stdin_stolen = matches!(args.seed, Some(None)) || matches!(args.ast, Some(None));

    let mut grammar = match args.file {
        Some(path) => Grammar::parse_from_file(path),
        None => {
            if !stdin_stolen {
                match io::read_stdin() {
                    Some(str) => Grammar::parse_from_str(&str),
                    None => Grammar::default(),
                }
            } else {
                Grammar::default()
            }
        }
    };

    if let Some(seed_opt) = args.seed {
        let seed_str = match seed_opt {
            Some(str) => str,
            None => io::read_stdin().unwrap_or("".to_owned()),
        };
        let seed = match U256::from_str(seed_str.trim()) {
            Ok(num) => num,
            Err(e) => {
                eprintln!(
                    "[ERROR]: Invalid seed supplied: \"{}\"\nDetails: {}",
                    seed_str, e
                );
                std::process::exit(1);
            }
        };

        rng::set_seed(seed);
    }

    if args.dump_seed {
        println!("SEED: {:x}", rng::get_seed())
    }

    if args.dump_grammar {
        println!("# CURRENT GRAMMAR\n{}", grammar);
    }

    let tree = generate::generate_tree(&mut grammar, args.depth);

    if args.dump_ast {
        println!("R:\n{}\nG:\n{}\nB:\n{}", tree.0, tree.1, tree.2);
    }

    let has_t = grammar.rules.iter().any(|x| x.0 == NodeType::T);

    let is_gif_ext = match &args.out {
        Some(path) => path.to_str().unwrap().to_lowercase().ends_with(".gif"),
        None => false,
    };

    if (args.out.is_none() && has_t) || is_gif_ext {
        img::gen_gif(
            args.out.unwrap_or(PathBuf::from_str("out.gif").unwrap()),
            args.width,
            args.height,
            args.frames,
            &tree,
        );
    } else {
        img::gen_img(
            args.out.unwrap_or(PathBuf::from_str("out.png").unwrap()),
            args.width,
            args.height,
            &tree,
        );
    }
}
