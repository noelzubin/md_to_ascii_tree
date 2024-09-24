use ascii_tree::{write_tree, Tree};
use parse::{parse_tree, FileStructure};
use std::io::stdin;

mod parse;

fn get_node(node: FileStructure) -> Tree {
    if node.children.is_empty() {
        return Tree::Leaf(vec![node.name]);
    } else {
        let children = node.children.into_iter().map(get_node).collect();
        return Tree::Node(node.name, children);
    }
}

fn main() {
    let tree_text = std::io::read_to_string(stdin()).unwrap();
    let root = parse_tree(&tree_text);

    let mut output = String::new();
    for node in root.children.into_iter() {
        write_tree(&mut output, &get_node(node)).unwrap();
    }
    println!("{}", output);
}
