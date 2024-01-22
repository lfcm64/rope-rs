use super::nodes::{inner::Inner, leaf::Leaf, node::Node};
use std::str;

pub struct RopeBuilder<'a> {
    root: &'a mut Node,
    stack: Vec<Node>,
}

impl<'a> RopeBuilder<'a> {
    pub fn new(root: &'a mut Node) -> Self {
        RopeBuilder {
            root,
            stack: Vec::new(),
        }
    }

    pub fn add(&mut self, buf: &[u8]) {
        let str = str::from_utf8(buf).unwrap();
        self.add_node(Leaf::new(str));
    }

    fn add_node(&mut self, leaf: Leaf) {
        let mut node = Node::Leaf(leaf);

        while let Some(last) = self.stack.pop() {
            match (&last, &node) {
                (Node::Leaf(..), _) => {
                    node = Node::Inner(Inner::complete_new(last, node));
                }
                (Node::Inner(..), Node::Leaf(..)) => {
                    self.stack.push(last);
                    break;
                }
                (Node::Inner(..), Node::Inner(..)) if node.r_depth() == last.r_depth() => {
                    node = Node::Inner(Inner::complete_new(last, node));
                }
                _ => {
                    self.stack.push(last);
                    break;
                }
            }
        }

        self.stack.push(node);
    }
    pub fn finish(&mut self) {
        if self.stack.len() > 1 {
            let mut right = self.stack.pop().unwrap();

            while self.stack.len() > 0 {
                let left = self.stack.pop().unwrap();
                match (&right, &left) {
                    (Node::Leaf(..), Node::Leaf(..)) => {
                        right = Node::Inner(Inner::complete_new(left, right));
                    }
                    (Node::Leaf(..), Node::Inner(..)) => {
                        right = Node::Inner(Inner::vacant_new(right, true));
                        self.stack.push(left);
                    }
                    (Node::Inner(..), _) if right.r_depth() == left.r_depth() => {
                        right = Node::Inner(Inner::complete_new(left, right));
                    }
                    (Node::Inner(..), _) => {
                        right = Node::Inner(Inner::vacant_new(right, true));
                        self.stack.push(left);
                    }
                }
            }

            *self.root = right;
        } else {
            *self.root = self.stack.pop().unwrap();
        }
    }
}
