use node::{
    NodeType,
    generate::{self, Grammar},
};

mod img;
pub mod node;

fn main() {
    let mut grammar = Grammar::new(vec![
        (NodeType::X, 1),
        (NodeType::Y, 1),
        (NodeType::Literal, 1),
        //(NodeType::Rand, 1),
        //(NodeType::Add, 4),
        //(NodeType::Sub, 4),
        (NodeType::Mult, 4),
        (NodeType::Div, 4),
        (NodeType::Sqrt, 3),
        //(NodeType::Pow, 3),
        (NodeType::Max, 3),
        (NodeType::Min, 3),
        (NodeType::Sin, 5),
        (NodeType::Tan, 5),
        //(NodeType::Cos, 5),
        (NodeType::Abs, 1),
        (NodeType::If, 1),
    ]);

    let tree = generate::generate_tree(&mut grammar, 10);
    println!("## R:\n{}\n## G:\n{}\n## B:\n{}", tree.0, tree.1, tree.2);

    let img = img::gen_img(512, 512, &tree);

    img.save("test.png").unwrap();
}
