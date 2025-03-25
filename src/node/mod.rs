pub mod generate;

use std::fmt::Display;

use crate::grammar::Grammar;
use rand::{Rng, seq::IndexedRandom};

pub type NodeTree = (NodePtr, NodePtr, NodePtr);

pub type NodePtr = Box<Node>;

/// A simple enum which holds the types of nodes available
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeType {
    /// The x value of the current pixel
    X,
    /// The y value of the current pixel
    Y,
    /// The current time. Goes from 0 to PI. Defaults to 0 if not in gif mode
    T,
    /// A random value in the range `-1..=1`
    Rand,
    /// A float literal
    Literal,
    /// Multiply two values
    Mult,
    /// Add two values
    Add,
    /// Subtract two values
    Sub,
    /// Divide a value with another
    Div,
    /// Raise a value to the power of another
    Pow,
    /// Take the square root of a value
    Sqrt,
    /// Mods one value with another
    Mod,
    /// Get the max value of two values
    Max,
    /// Get the minimum value of two values
    Min,
    /// Applies the `sin` function on the value
    Sin,
    /// Applies the `cos` function on the value
    Cos,
    /// Applies the `tan` function on the value
    Tan,
    /// Takes the absolute value of a value
    Abs,
    /// A simple if statement
    If,
}

impl NodeType {
    /// If the current node doesn't have child branches, and can therefore be collapsed
    pub fn is_end(&self) -> bool {
        matches!(self, Self::X | Self::Y | Self::Rand | Self::Literal)
    }
}

impl TryFrom<&str> for NodeType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "t" => Ok(Self::T),
            "rand" => Ok(Self::Rand),
            "literal" => Ok(Self::Literal),
            "mult" => Ok(Self::Mult),
            "add" => Ok(Self::Add),
            "sub" => Ok(Self::Sub),
            "div" => Ok(Self::Div),
            "pow" => Ok(Self::Pow),
            "sqrt" => Ok(Self::Sqrt),
            "mod" => Ok(Self::Mod),
            "max" => Ok(Self::Max),
            "min" => Ok(Self::Min),
            "sin" => Ok(Self::Sin),
            "cos" => Ok(Self::Cos),
            "tan" => Ok(Self::Tan),
            "abs" => Ok(Self::Abs),
            "if" => Ok(Self::If),
            _ => Err(()),
        }
    }
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            NodeType::X => "x",
            NodeType::Y => "y",
            NodeType::T => "t",
            NodeType::Rand => "rand",
            NodeType::Literal => "literal",
            NodeType::Mult => "mult",
            NodeType::Add => "add",
            NodeType::Sub => "sub",
            NodeType::Div => "div",
            NodeType::Pow => "pow",
            NodeType::Sqrt => "sqrt",
            NodeType::Mod => "mod",
            NodeType::Max => "max",
            NodeType::Min => "min",
            NodeType::Sin => "sin",
            NodeType::Cos => "cos",
            NodeType::Tan => "tan",
            NodeType::Abs => "abs",
            NodeType::If => "if",
        };
        write!(f, "{}", name)
    }
}

/// A node which will form a tree, that can be collapsed into a single value
#[derive(Clone, Debug)]
pub enum Node {
    /// The x value of the current pixel
    X,
    /// The y value of the current pixel
    Y,
    /// The current time. Goes from 0 to PI. Defaults to 0 if not in gif mode
    T,
    /// A random value in the range `-1..=1`. Picked at run time
    Rand,
    /// A float literal. Picked randomly at creation time
    Literal(f64),
    /// Multiply two values
    Mult(NodePtr, NodePtr),
    /// Add two values
    Add(NodePtr, NodePtr),
    /// Subtract two values
    Sub(NodePtr, NodePtr),
    /// Divide a value with another
    Div(NodePtr, NodePtr),
    /// Raise a value to the power of another
    Pow(NodePtr, NodePtr),
    /// Take the square root of a value
    Sqrt(NodePtr),
    /// Mods one value with another
    Mod(NodePtr, NodePtr),
    /// Get the max value of two values
    Max(NodePtr, NodePtr),
    /// Get the minimum value of two values
    Min(NodePtr, NodePtr),
    /// Applies the `sin` function on the value
    Sin(NodePtr),
    /// Applies the `cos` function on the value
    Cos(NodePtr),
    /// Applies the `tan` function on the value
    Tan(NodePtr),
    /// Takes the absolute value of a value
    Abs(NodePtr),
    /// A simple if statement
    If(IfNode),
}

impl Node {
    /// If the current node doesn't have child branches, and can therefore be collapsed
    pub fn is_end(&self) -> bool {
        matches!(self, Self::X | Self::Y | Self::Rand | Self::Literal(_))
    }

    /// Collapse this branch into a value
    pub fn get_value(&self, x: f64, y: f64, t: f64) -> f64 {
        match self {
            Node::X => x,
            Node::Y => y,
            Node::T => t,
            Node::Rand => {
                let mut rng = rand::rng();
                rng.random_range(-1.0..=1.0)
            }
            Node::Literal(float) => *float,
            Node::Mult(lhs, rhs) => lhs.get_value(x, y, t) * rhs.get_value(x, y, t),
            Node::Add(rhs, lhs) => lhs.get_value(x, y, t) + rhs.get_value(x, y, t),
            Node::Sub(rhs, lhs) => lhs.get_value(x, y, t) - rhs.get_value(x, y, t),
            Node::Div(lhs, rhs) => {
                let rhs_value = rhs.get_value(x, y, t);
                lhs.get_value(x, y, t)
                    / if rhs_value != 0. {
                        rhs_value
                    } else {
                        f64::EPSILON
                    }
            }
            Node::Pow(lhs, rhs) => lhs.get_value(x, y, t).powf(rhs.get_value(x, y, t)),
            Node::Sqrt(val) => val.get_value(x, y, t).sqrt(),
            Node::Mod(lhs, rhs) => lhs.get_value(x, y, t) % rhs.get_value(x, y, t),
            Node::Max(lhs, rhs) => lhs.get_value(x, y, t).max(rhs.get_value(x, y, t)),
            Node::Min(lhs, rhs) => lhs.get_value(x, y, t).min(rhs.get_value(x, y, t)),
            Node::Sin(val) => val.get_value(x, y, t).sin(),
            Node::Cos(val) => val.get_value(x, y, t).cos(),
            Node::Tan(val) => val.get_value(x, y, t).tan(),
            Node::Abs(val) => val.get_value(x, y, t).abs(),
            Node::If(if_node) => {
                if if_node.operator.eval(
                    if_node.lhs.get_value(x, y, t),
                    if_node.rhs.get_value(x, y, t),
                ) {
                    if_node.on_true.get_value(x, y, t)
                } else {
                    if_node.on_false.get_value(x, y, t)
                }
            }
        }
    }

    /// Get a random terminable node.
    pub fn get_rand_end(grammar: &mut Grammar) -> NodePtr {
        let ends = grammar
            .rules
            .iter()
            .filter_map(|x| x.0.is_end().then_some(x.0))
            .collect::<Vec<_>>();

        let Some(choice) = ends.choose(&mut grammar.rng) else {
            eprintln!("[ERROR]: Grammar needs to include at least one element that is terminable");
            std::process::exit(1);
        };

        match choice {
            NodeType::X => Box::new(Self::X),
            NodeType::Y => Box::new(Self::Y),
            NodeType::Rand => Box::new(Self::Rand),
            NodeType::Literal => Box::new(Self::Literal(grammar.rng.random_range(-1.0..=1.0))),
            _ => unreachable!(),
        }
    }

    pub fn gen_rand(grammar: &mut Grammar, curr_depth: usize) -> NodePtr {
        if curr_depth == 0 {
            return Self::get_rand_end(grammar);
        }

        let choice = grammar.pick();

        let new_depth = curr_depth - 1;

        let mut gen_node = || Self::gen_rand(grammar, new_depth);

        let node = match choice {
            NodeType::T => Node::T,
            NodeType::X => Node::X,
            NodeType::Y => Node::Y,
            NodeType::Rand => Node::Rand,
            NodeType::Literal => Node::Literal(grammar.rng.random_range(-1.0..=1.0)),
            NodeType::Mult => Node::Mult(gen_node(), gen_node()),
            NodeType::Add => Node::Add(gen_node(), gen_node()),
            NodeType::Sub => Node::Sub(gen_node(), gen_node()),
            NodeType::Div => Node::Div(gen_node(), gen_node()),
            NodeType::Pow => Node::Pow(gen_node(), gen_node()),
            NodeType::Sqrt => Node::Sqrt(gen_node()),
            NodeType::Mod => Node::Mod(gen_node(), gen_node()),
            NodeType::Max => Node::Max(gen_node(), gen_node()),
            NodeType::Min => Node::Min(gen_node(), gen_node()),
            NodeType::Sin => Node::Sin(gen_node()),
            NodeType::Cos => Node::Cos(gen_node()),
            NodeType::Tan => Node::Tan(gen_node()),
            NodeType::Abs => Node::Abs(gen_node()),
            NodeType::If => Node::If(IfNode {
                lhs: gen_node(),
                rhs: gen_node(),
                operator: Operator::pick_rand(),
                on_true: gen_node(),
                on_false: gen_node(),
            }),
        };

        Box::new(node)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::X => write!(f, "x"),
            Node::Y => write!(f, "y"),
            Node::T => write!(f, "t"),
            Node::Rand => write!(f, "RAND"),
            Node::Literal(float) => write!(f, "{}", float),
            Node::Mult(lhs, rhs) => write!(f, "mult({}, {})", lhs, rhs),
            Node::Add(lhs, rhs) => write!(f, "add({}, {})", lhs, rhs),
            Node::Sub(lhs, rhs) => write!(f, "sub({}, {})", lhs, rhs),
            Node::Div(lhs, rhs) => write!(f, "div({}, {})", lhs, rhs),
            Node::Pow(lhs, rhs) => write!(f, "pow({}, {})", lhs, rhs),
            Node::Sqrt(val) => write!(f, "sqrt({})", val),
            Node::Mod(lhs, rhs) => write!(f, "mod({}, {})", lhs, rhs),
            Node::Max(lhs, rhs) => write!(f, "max({}, {})", lhs, rhs),
            Node::Min(lhs, rhs) => write!(f, "min({}, {})", lhs, rhs),
            Node::Sin(val) => write!(f, "sin({})", val),
            Node::Cos(val) => write!(f, "cos({})", val),
            Node::Tan(val) => write!(f, "tan({})", val),
            Node::Abs(val) => write!(f, "abs({})", val),
            Node::If(if_node) => write!(
                f,
                "({} {} {} : {} ? {})",
                if_node.lhs, if_node.operator, if_node.rhs, if_node.on_true, if_node.on_false
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IfNode {
    /// The first operand
    lhs: NodePtr,
    /// The second operand
    rhs: NodePtr,
    /// The operator to be applied to the operands
    operator: Operator,
    /// The value that will be used if the expression is true
    on_true: NodePtr,
    /// The value that will be used if the expression is false
    on_false: NodePtr,
}

#[derive(Clone, Debug)]
pub enum Operator {
    /// `lhs < rhs`
    LessThan,
    /// `lhs > rhs`
    GreaterThan,
    /// `lhs == rhs`
    Equals,
    /// `lhs != rhs`
    NotEquals,
}

impl Operator {
    pub fn eval(&self, lhs: f64, rhs: f64) -> bool {
        match self {
            Self::LessThan => lhs < rhs,
            Self::GreaterThan => lhs > rhs,
            Self::Equals => lhs == rhs,
            Self::NotEquals => lhs == rhs,
        }
    }

    pub fn pick_rand() -> Self {
        const COUNT: u8 = 4;

        let mut rng = rand::rng();
        match rng.random_range(0..COUNT) {
            0 => Self::LessThan,
            1 => Self::GreaterThan,
            2 => Self::Equals,
            3 => Self::NotEquals,
            _ => unreachable!(),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LessThan => write!(f, "<"),
            Self::GreaterThan => write!(f, ">"),
            Self::Equals => write!(f, "=="),
            Self::NotEquals => write!(f, "!="),
        }
    }
}
