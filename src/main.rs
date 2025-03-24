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
        (NodeType::Literal, 2),
        (NodeType::Add, 3),
        (NodeType::Mult, 3),
    ]);

    let tree = generate::generate_tree(&mut grammar, 4);
    println!("## R:\n{}\n## G:\n{}\n## B:\n{}", tree.0, tree.1, tree.2);

    let img = img::gen_img(255, 255, &tree);

    img.save("test.png").unwrap();
}
