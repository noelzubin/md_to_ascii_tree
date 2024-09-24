use regex::Regex;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileStructure {
    pub name: String,
    pub children: Vec<FileStructure>,
    indent_count: isize,
}

fn create_node(
    mut node: FileStructure,
    iter: &mut Peekable<impl Iterator<Item = FileStructure>>,
) -> FileStructure {
    if let None = iter.peek() { // No more nodes to process
        return node;
    }

    while let Some(next) = iter.peek() {
        if next.indent_count <= node.indent_count {
            break; // No more children to process
        } else {
            let next = iter.next().unwrap();
            node.children.push(create_node(next, iter));
        }
    }

    node
}

pub fn parse_tree(input: &str) -> FileStructure {
    let structures = split_input(input);
    let mut structures = structures.into_iter().peekable();
    let root = FileStructure {
        name: "root".to_string(),
        children: vec![],
        indent_count: -1,
    };
    return create_node(root, &mut structures);
}

fn split_input(input: &str) -> Vec<FileStructure> {
    // Matches the whitespace in front of a file name.
    // Also will match a markdown bullet point if included.
    // For example, testing against "  - hello" will return
    // a positive match with the first capturing group
    //  with "  - " and a second with "  "
    let leading_whitespace_and_bullet_re = regex::Regex::new(r"^(\s*)([\*\-]\s*)?").unwrap();

    // Matches lines containing only whitespace
    let only_whitespace_re = Regex::new(r"^\s*$").unwrap();

    return input
        .lines()
        .filter(|line| !only_whitespace_re.is_match(line))
        .map(|line| {
            let result = leading_whitespace_and_bullet_re.captures(line).unwrap();
            let indent_count = result.get(1).unwrap().as_str().len();
            let name = leading_whitespace_and_bullet_re
                .replace(line, "")
                .to_string();

            return FileStructure {
                name,
                children: vec![],
                indent_count: indent_count as isize,
            };
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_fs(name: &str, children: Vec<FileStructure>, indent_count: isize) -> FileStructure {
        FileStructure {
            name: name.to_string(),
            children,
            indent_count,
        }
    }

    #[test]
    fn one_line() {
        let result = parse_tree("42");
        assert_eq!(
            dbg!(result),
            new_fs("root", vec![new_fs("42", vec![], 0)], -1)
        );
    }

    #[test]
    fn with_nesting() {
        let result = parse_tree("1\n 2\n 3");
        assert_eq!(
            dbg!(result),
            new_fs(
                "root",
                vec![new_fs(
                    "1",
                    vec![new_fs("2", vec![], 1), new_fs("3", vec![], 1)],
                    0
                )],
                -1
            )
        );
    }

    #[test]
    fn complex() {
        let result = parse_tree("1\n 2\n 3\n  4\n  5\n 6\n 7\n8\n 9");
        assert_eq!(
            dbg!(result),
            new_fs(
                "root",
                vec![
                    new_fs(
                        "1",
                        vec![
                            new_fs("2", vec![], 1),
                            new_fs("3", vec![new_fs("4", vec![], 2), new_fs("5", vec![], 2)], 1),
                            new_fs("6", vec![], 1),
                            new_fs("7", vec![], 1),
                        ],
                        0
                    ),
                    new_fs("8", vec![new_fs("9", vec![], 1)], 0),
                ],
                -1
            )
        );
    }
}
