use std::{fs::OpenOptions, io::Read, path::PathBuf, str::FromStr};

use clap::Parser;
use grammar::Grammar;
use node::{NodeType, ast};
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

    if matches!(args.seed, Some(None)) && matches!(args.ast, Some(None)) {
        eprintln!(
            "[ERROR]: Both --seed and --ast are trying to read from STDIN. Only one is allowed at a time"
        );
        std::process::exit(1)
    }

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

    let ast = {
        if let Some(ast_opt) = args.ast {
            let ast_str = match ast_opt {
                Some(path) => {
                    let Ok(mut file) = OpenOptions::new().read(true).open(path.clone()) else {
                        eprintln!("[ERROR]: Failed to open AST file {:?}", path);
                        std::process::exit(1)
                    };

                    let mut buf = String::new();
                    _ = file.read_to_string(&mut buf);
                    buf
                }
                None => io::read_stdin().unwrap_or("".to_owned()),
            };
            ast::NodeAst::parse_from_str(&ast_str)
        } else {
            ast::NodeAst::from_grammar(&mut grammar, args.depth)
        }
    };

    if args.dump_seed {
        println!("SEED: {:x}", rng::get_seed())
    }

    if args.dump_grammar {
        println!("# CURRENT GRAMMAR\n{}", grammar);
    }

    if args.dump_ast {
        println!("R:\n{}\nG:\n{}\nB:\n{}", ast.r, ast.g, ast.b);
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
            &ast,
        );
    } else {
        img::gen_img(
            args.out.unwrap_or(PathBuf::from_str("out.png").unwrap()),
            args.width,
            args.height,
            &ast,
        );
    }
}
