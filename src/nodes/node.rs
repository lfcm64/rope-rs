use super::{inner::Inner, leaf::Leaf};
use std::fmt;

#[derive(Clone)]
pub enum Node {
    Inner(Inner),
    Leaf(Leaf),
}

impl Node {
    pub fn r_depth(&self) -> usize {
        match self {
            Self::Inner(inner) => inner.metadata.r_depth,
            Self::Leaf(..) => 0,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Inner(inner) => inner.metadata.n_char,
            Self::Leaf(leaf) => leaf.metadata.n_char,
        }
    }

    pub fn line_break(&self) -> usize {
        match self {
            Self::Inner(inner) => inner.metadata.n_line_break,
            Self::Leaf(leaf) => leaf.metadata.n_line_break,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Inner(inner) => write!(f, "{}", inner),
            Node::Leaf(leaf) => write!(f, "{}", leaf),
        }
    }
}