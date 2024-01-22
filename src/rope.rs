use super::{
    builder::RopeBuilder,
    nodes::{leaf::Leaf, node::Node},
};
use std::{
    fmt, fs,
    io::{self, BufRead, BufReader},
    path,
};

const BUF_SIZE: usize = 8_000;

pub struct Rope {
    root: Node,
}

impl Rope {
    pub fn new() -> Self {
        Self {
            root: Node::Leaf(Leaf::new("")),
        }
    }

    pub fn from_file(mut self, path: &path::Path) -> Result<Self, io::Error> {
        let file = fs::File::open(path)?;
        let mut builder = RopeBuilder::new(&mut self.root);

        let mut reader = BufReader::with_capacity(BUF_SIZE, file);

        loop {
            let lenght = {
                let buffer = reader.fill_buf()?;
                builder.add(buffer);
                buffer.len()
            };

            if lenght != BUF_SIZE {
                break;
            }
            reader.consume(lenght);
        }
        builder.finish();

        Ok(self)
    }

    pub fn leaf_number(&self) -> usize {
        fn count_leaf(node: Option<&Node>) -> usize {
            if let Some(n) = node {
                match n {
                    Node::Inner(inner) => count_leaf(inner.left()) + count_leaf(inner.right()),
                    Node::Leaf(_) => 1,
                }
            } else {
                0
            }
        }
        count_leaf(Some(&self.root))
    }

    pub fn line(&self, l_index: usize) -> Option<String> {
        fn traverse(node: &Node, index: usize) -> Option<String> {
            match node {
                Node::Inner(inner) => {
                    if let (Some(l), Some(r)) = (inner.left(), inner.right()) {
                        let left_result = index <= l.line_break();
                        let right_result =
                            index >= l.line_break() && index <= l.line_break() + r.line_break();

                        return match (left_result, right_result) {
                            (true, true) => {
                                Some(traverse(l, index)? + &traverse(r, index - l.line_break())?)
                            }
                            (true, _) => traverse(l, index),
                            (_, true) => traverse(r, index - l.line_break()),
                            _ => None,
                        };
                    } else if let Some(l) = inner.left() {
                        if l.line_break() >= index {
                            return traverse(l, index);
                        }
                    } else if let Some(r) = inner.right() {
                        if r.line_break() >= index {
                            return traverse(r, index);
                        }
                    }
                    None
                }
                Node::Leaf(leaf) => leaf.line(index),
            }
        }
        traverse(&self.root, l_index)
    }

    pub fn insert(&self) {}

    pub fn delete(&self) {}

    pub fn save(&self) {}
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}
