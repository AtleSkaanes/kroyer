use std::io::Write;

use crate::node::{self, IfNode, Node, NodePtr, NodeType, Operator};

use super::NodeAst;

/// A token representing a piece of the AST getting parsed
#[derive(Clone, Debug, PartialEq)]
enum AstToken {
    /// A identifier, like `mult` and `x`
    Ident(String),
    /// A float literal, like `3.14`
    Literal(f64),
    /// A section header. This is used to signify what parts of the AST are used for what color
    /// value.
    /// Like:
    /// ```
    /// R: // <- Section header
    /// <ast>
    /// G:
    /// <ast>
    /// B:
    /// <ast>
    /// ```
    SectionHeader(char),
    /// A open bracket: `(`
    BracketOpen,
    /// A closed bracket: `)`
    BracketEnd,
    /// An operator used for comparions, like `>` and `==`
    Operator(node::Operator),
    /// Signifies that the next part should be used as the `on_true` part of the if statement
    IfThen,
    /// Signifies that the next part should be used as the `on_false` part of the if statement
    Else,
    /// An unknown string
    Unknown(String),
    /// The end of the file
    Eof,
}

impl AstToken {
    pub fn to_node(&self, parser: &mut AstParser) -> Option<NodePtr> {
        match self {
            AstToken::Ident(ident) => {
                let Ok(parent) = NodeType::try_from(ident.as_str()) else {
                    eprintln!(
                        "[ERROR]: Whilst parsing AST, got invalid identifier \"{}\"",
                        ident
                    );
                    std::process::exit(1);
                };

                let node = node_from_token_stream(parent, parser);
                Some(Box::new(node))
            }
            AstToken::Literal(literal) => Some(Box::new(Node::Literal(*literal))),
            _ => None,
        }
    }
}

fn parse_if_statement(lhs: NodePtr, parser: &mut AstParser) -> Option<NodePtr> {
    let AstToken::Operator(operator) = parser.peek() else {
        return None;
    };
    _ = parser.next_token();

    let Some(rhs) = parser.next_token().to_node(parser) else {
        eprintln!(
            "[ERROR]: Whilst parsing AST, expected right hand side argument for operator {}, got \"{:?}\"",
            operator,
            parser.get_current_token()
        );
        std::process::exit(1)
    };

    if parser.next_token() != AstToken::IfThen {
        eprintln!(
            "[ERROR]: Whilst parsing AST, expected \"?\" in if statement, got \"{:?}\"",
            parser.get_current_token()
        );
        std::process::exit(1)
    }

    let Some(on_true) = parser.next_token().to_node(parser) else {
        eprintln!(
            "[ERROR]: Whilst parsing AST, expected literal or identifier in if statement, got \"{:?}\"",
            parser.get_current_token()
        );
        std::process::exit(1)
    };

    if parser.next_token() != AstToken::Else {
        eprintln!(
            "[ERROR]: Whilst parsing AST, expected \":\" in if statement, got \"{:?}\"",
            parser.get_current_token()
        );
        std::process::exit(1)
    }

    let Some(on_false) = parser.next_token().to_node(parser) else {
        eprintln!(
            "[ERROR]: Whilst parsing AST, expected literal or identifier as else statement, got \"{:?}\"",
            parser.get_current_token()
        );
        std::process::exit(1)
    };

    let if_node = IfNode {
        lhs,
        rhs,
        operator: operator.clone(),
        on_true,
        on_false,
    };

    Some(Box::new(Node::If(if_node)))
}

fn node_from_token_stream(parent: NodeType, parser: &mut AstParser) -> Node {
    let num_args = parent.arg_num();

    let mut args: Vec<NodePtr> = Vec::with_capacity(num_args);

    if num_args >= 1 {
        let first_tok = parser.next_token();

        if first_tok == AstToken::Eof {
            eprintln!(
                "[ERROR]: Whilst parsing AST, expected parameter list for \"{}\", got EOF",
                parent
            );
        }

        if !matches!(first_tok, AstToken::BracketOpen) {
            eprintln!(
                "[ERROR]: Whilst parsing AST, expected param list for {}, got {:?}",
                parent, first_tok
            );
            std::process::exit(1);
        }

        while parser.peek() != AstToken::Eof {
            let tok = parser.next_token();
            match &tok {
                AstToken::Ident(ident) => {
                    let Ok(node_type) = NodeType::try_from(ident.as_str()) else {
                        eprintln!(
                            "[ERROR]: Whilst parsing AST, invalid identifier \"{}\"",
                            ident
                        );
                        std::process::exit(1);
                    };

                    args.push(Box::new(node_from_token_stream(node_type, parser)));
                }
                AstToken::Literal(literal) => {
                    let node = Node::Literal(*literal);
                    args.push(Box::new(node));
                }
                AstToken::SectionHeader(header) => {
                    eprintln!(
                        "[ERROR]: Whilst parsing AST, expected parameter for {}, found section header '{}'",
                        parent, header
                    );
                    std::process::exit(1);
                }
                AstToken::BracketOpen => continue,
                AstToken::BracketEnd => {
                    break;
                }
                AstToken::Operator(op) => {
                    eprintln!(
                        "[ERROR]: Whilst parsing AST, expected parameter for {}, found out of place operator \"{}\"",
                        parent, op
                    )
                }
                AstToken::IfThen => {
                    eprintln!(
                        "[ERROR]: Whilst parsing AST, expected parameter for {}, found out of place '?'",
                        parent,
                    );
                    std::process::exit(1);
                }
                AstToken::Else => {
                    eprintln!(
                        "[ERROR]: Whilst parsing AST, expected parameter for {}, found out of place ':'",
                        parent,
                    );
                    std::process::exit(1);
                }
                AstToken::Unknown(str) => {
                    eprintln!(
                        "[ERROR]: Whilst parsing AST, expected parameter for {}, found unknown expression \"{}\"",
                        parent, str
                    );
                    std::process::exit(1);
                }
                AstToken::Eof => {
                    eprintln!(
                        "[ERROR]: Whilst parsing AST, expected parameter for {}, found EOF",
                        parent
                    );
                    std::process::exit(1)
                }
            }
        }
        _ = std::io::stdout().flush();
        if args.len() != num_args {
            eprintln!(
                "[ERROR]: Whilst parsing AST, {} expected {} paramaters, got {}: {:?}",
                parent,
                num_args,
                args.len(),
                args
            );
            std::process::exit(1);
        }
    }

    let node = match parent {
        NodeType::X => Node::X,
        NodeType::Y => Node::Y,
        NodeType::T => Node::T,
        NodeType::Rand => Node::Rand,
        // Literals should be handled in while loop, and should never be the parent
        NodeType::Literal => unreachable!(),
        NodeType::Mult => Node::Mult(args[0].clone(), args[1].clone()),
        NodeType::Add => Node::Add(args[0].clone(), args[1].clone()),
        NodeType::Sub => Node::Sub(args[0].clone(), args[1].clone()),
        NodeType::Div => Node::Div(args[0].clone(), args[1].clone()),
        NodeType::Pow => Node::Pow(args[0].clone(), args[1].clone()),
        NodeType::Sqrt => Node::Sqrt(args[0].clone()),
        NodeType::Mod => Node::Mod(args[0].clone(), args[1].clone()),
        NodeType::Max => Node::Max(args[0].clone(), args[1].clone()),
        NodeType::Min => Node::Min(args[0].clone(), args[1].clone()),
        NodeType::Sin => Node::Sin(args[0].clone()),
        NodeType::Cos => Node::Cos(args[0].clone()),
        NodeType::Tan => Node::Tan(args[0].clone()),
        NodeType::Abs => Node::Abs(args[0].clone()),
        NodeType::If => todo!(),
    };

    if let Some(if_node) = parse_if_statement(Box::new(node.clone()), parser) {
        return *if_node;
    }

    node
}

struct AstParser {
    index: usize,
    source: String,
    current_token: Option<AstToken>,
}

impl AstParser {
    pub fn new(source: &str) -> Self {
        Self {
            index: 0,
            source: source.to_owned(),
            current_token: None,
        }
    }

    pub fn get_current_token(&self) -> Option<AstToken> {
        self.current_token.clone()
    }

    pub fn peek(&mut self) -> AstToken {
        let idx = self.index;
        let token = self.next_token_inner();
        self.index = idx;
        token
    }

    pub fn next_token(&mut self) -> AstToken {
        let token = self.next_token_inner();
        self.current_token = Some(token.clone());
        token
    }

    fn next_token_inner(&mut self) -> AstToken {
        while let Some(ch) = self.source.chars().nth(self.index) {
            if ch.is_whitespace() || ",".contains(ch) {
                self.index += 1;
            } else if ch == '#' {
                // Comment
                while let Some(ch) = self.source.chars().nth(self.index) {
                    if ch == '\n' {
                        break;
                    }
                    self.index += 1;
                }
            } else {
                break;
            }
        }

        if let Some(ch) = self.source.chars().nth(self.index) {
            if ch == '(' {
                self.index += 1;
                return AstToken::BracketOpen;
            } else if ch == ')' {
                self.index += 1;
                return AstToken::BracketEnd;
            }
        } else {
            // EOF has been reached
            return AstToken::Eof;
        }

        let mut buf = String::new();

        while let Some(ch) = self.source.chars().nth(self.index) {
            // Token terminating charachters
            if "(),".contains(ch) || ch.is_whitespace() {
                break;
            }

            buf.push(ch);
            self.index += 1;
        }

        if buf.len() > 1 && buf.ends_with(':') {
            return AstToken::SectionHeader(buf.chars().next().unwrap());
        }

        if let Ok(num) = buf.parse::<f64>() {
            return AstToken::Literal(num);
        }

        if let Ok(op) = Operator::try_from(buf.as_str()) {
            return AstToken::Operator(op);
        }

        if buf == "?" {
            return AstToken::IfThen;
        }

        if buf == ":" {
            return AstToken::Else;
        }

        if buf.chars().all(char::is_alphanumeric) {
            return AstToken::Ident(buf);
        }

        AstToken::Unknown(buf)
    }
}

impl NodeAst {
    pub fn parse_from_str(str: &str) -> Self {
        let mut parser = AstParser::new(str);

        let mut curr_header = ' ';

        let mut r_ast: Option<NodePtr> = None;
        let mut g_ast: Option<NodePtr> = None;
        let mut b_ast: Option<NodePtr> = None;

        let mut set_ast = |node: NodePtr, header: char| {
            match header {
                'r' => r_ast = Some(node),
                'g' => g_ast = Some(node),
                'b' => b_ast = Some(node),
                _ => {
                    eprintln!("[ERROR]: Whilst parsing AST, got expression outside header segment");
                    std::process::exit(1)
                }
            };
        };

        while parser.peek() != AstToken::Eof {
            let tok = parser.next_token();
            match tok {
                AstToken::Ident(ident) => {
                    let Ok(parent) = NodeType::try_from(ident.as_str()) else {
                        eprintln!(
                            "[ERROR]: Whilst parsing AST, got invalid identifier \"{}\"",
                            ident
                        );
                        std::process::exit(1);
                    };

                    let node = Box::new(node_from_token_stream(parent, &mut parser));

                    set_ast(node, curr_header);
                }
                AstToken::Literal(literal) => {
                    let node = Box::new(Node::Literal(literal));
                    set_ast(node, curr_header)
                }
                AstToken::SectionHeader(header) => {
                    let lower_header = header.to_lowercase().next().unwrap();

                    if curr_header == lower_header {
                        eprintln!(
                            "[ERROR]: Whilst parsing AST, encountered duplicate header '{}'",
                            header
                        );
                        std::process::exit(1);
                    }

                    if !"rgb".contains(lower_header) {
                        eprintln!(
                            "[ERROR]: Whilst parsing AST, invalid header '{}'. Headers can only be 'r', 'g', or 'b'",
                            header
                        )
                    }

                    curr_header = lower_header;
                }
                AstToken::BracketOpen => todo!(),
                AstToken::BracketEnd => todo!(),
                AstToken::Operator(_) => todo!(),
                AstToken::IfThen => todo!(),
                AstToken::Else => todo!(),
                AstToken::Unknown(ident) => {
                    eprintln!(
                        "[ERROR]: Whilst parsing AST, found unknown identifier \"{}\"",
                        ident
                    );
                    std::process::exit(1)
                }
                AstToken::Eof => todo!(),
            }
        }

        if r_ast.is_none() {
            eprintln!("[ERROR]: Whilst parsing AST, no AST for the r value was supplied");
            std::process::exit(1)
        }
        if g_ast.is_none() {
            eprintln!("[ERROR]: Whilst parsing AST, no AST for the g value was supplied");
            std::process::exit(1)
        }
        if b_ast.is_none() {
            eprintln!("[ERROR]: Whilst parsing AST, no AST for the b value was supplied");
            std::process::exit(1)
        }

        Self {
            r: r_ast.unwrap(),
            g: g_ast.unwrap(),
            b: b_ast.unwrap(),
        }
    }
}
