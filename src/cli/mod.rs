use std::path::PathBuf;

use clap::Parser;

/// Kroyer is a program used to create random pictures from a grammar file.
/// It has barely any practical use cases, but can be fun to tinker around with.
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// The grammar file to use. If none is used, it will use the default grammar.
    /// Convention is to use a file with the .kroyer file extension as the grammar file, but this
    /// convention is just made up by the author, and can be ignored without issue.
    /// Use --dump-default-grammar to view the default grammar
    file: Option<PathBuf>,
    /// Use a given seed. This assures that two images using the same grammar, and same seed, are
    /// identical
    #[arg(long)]
    seed: Option<u64>,
    /// Dumps the seed used to create the image into STDOUT. This can be passed to kroyer with --seed
    /// to create the same image again
    #[arg(long)]
    dump_seed: bool,
    /// Dumps the AST used to create the image into STDOUT.
    /// To create this exact image, this can be passed to kroyer with the --use-ast flag,
    /// either via STDIN or via a file
    #[arg(long)]
    dump_ast: bool,
    /// Dumps kroyers default grammar into STDOUT.
    /// This flag will stop all other processes, and will not create an image.
    #[arg(long)]
    dump_default_grammar: bool,
    /// Dumps the current grammar into STDOUT.
    #[arg(long)]
    dump_grammar: bool,
    /// Use a string as the grammar instead of a file.
    /// A string can also be passed via STDIN without needing to set this flag
    #[arg(short, long)]
    grammar: Option<String>,
    /// Creates a gif instead of a static png.
    /// If not in this mode, T will be evaluated to 0.
    /// This can also be implicitaly applied to the program by passing -o NAME.gif, without needing
    /// to use this flag
    #[arg(long)]
    gif: bool,
    /// Sets the path of the outputted image. Will default to out.png or out.gif, depending on the
    /// current mode.
    /// This can also be used to implicitally tell kroyer if it needs to use gif mode, by setting
    /// the file extension to `.gif`
    #[arg(short, long)]
    out: Option<PathBuf>,
    /// Dumps the raw image bytes into STDOUT instead of saving it to a file
    #[arg(long)]
    dump_raw: bool,
    /// Makes kroyer output more logs, which otherwise would be witheld.
    #[arg(short, long)]
    verbose: bool,
}
