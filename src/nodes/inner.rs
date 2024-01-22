use super::{metadata::Metadata, node::Node};
use std::fmt;

#[derive(Clone)]
pub struct Inner {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    pub metadata: Metadata,
}

impl Inner {
    pub fn complete_new(l: Node, r: Node) -> Self {
        let metadata = Metadata::from_children(Some(&l), Some(&r));

        Inner {
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
            metadata,
        }
    }

    pub fn vacant_new(node: Node, right_is_vacant: bool) -> Self {
        if right_is_vacant {
            return Inner {
                metadata: Metadata::from_children(Some(&node), None),
                left: Some(Box::new(node)),
                right: None,
            };
        }
        return Inner {
            metadata: Metadata::from_children(None, Some(&node)),
            left: None,
            right: Some(Box::new(node)),
        };
    }

    pub fn is_complete(&self) -> bool {
        if self.left.is_some() && self.right.is_some() {
            return true
        }
        false
    }

    pub fn left(&self) -> Option<&Node> {
        self.left.as_ref().map(|boxed_node| &**boxed_node)
    }

    pub fn right(&self) -> Option<&Node> {
        self.right.as_ref().map(|boxed_node| &**boxed_node)
    }
}

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(left_node) = &self.left {
            write!(f, "{}", left_node)?;
        }
        if let Some(right_node) = &self.right {
            write!(f, "{}", right_node)?;
        }
        Ok(())
    }
}