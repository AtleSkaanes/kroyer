use std::{
    io::{BufRead, IsTerminal},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;
use grammar::Grammar;
use node::{NodeType, generate};
use primitive_types::U256;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

mod cli;
pub mod grammar;
mod img;
pub mod node;

fn main() {
    let args = cli::Args::parse();

    // Handle flags that cancel all other operations
    if args.dump_default_grammar {
        print!("# DEFAULT GRAMMAR\n\n{}", Grammar::default());
        std::process::exit(0);
    }

    let mut grammar = match args.file {
        Some(path) => Grammar::parse_from_file(path),
        None => {
            if !std::io::stdin().is_terminal() {
                let str = std::io::stdin()
                    .lock()
                    .lines()
                    .fold(String::new(), |acc, line| {
                        acc + &line.unwrap_or_default() + "\n"
                    });
                Grammar::parse_from_str(&str)
            } else {
                Grammar::default()
            }
        }
    };

    if let Some(seed_str) = args.seed {
        let seed = match U256::from_str(&seed_str) {
            Ok(num) => num,
            Err(e) => {
                eprintln!(
                    "[ERROR]: Invalid seed supplied: \"{}\"\nDetails: {}",
                    seed_str, e
                );
                std::process::exit(1);
            }
        };

        grammar.rng = ChaCha20Rng::from_seed(seed.to_little_endian());
    }

    if args.dump_seed {
        println!(
            "SEED: {:x}",
            U256::from_little_endian(&grammar.rng.get_seed())
        )
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
