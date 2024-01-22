use super::node::Node;

#[derive(Clone)]
pub struct Metadata {
    pub n_line_break: usize,
    pub n_char: usize,
    pub r_depth: usize,
}

impl Metadata {
    pub fn new(n_line_break: usize, n_char: usize, r_depth: usize) -> Self {
        Metadata {
            n_line_break,
            n_char,
            r_depth,
        }
    }

    pub fn from_children(left_child: Option<&Node>, right_child: Option<&Node>) -> Self {
        let (n_line_break, n_char, node) = match (left_child, right_child) {
            (Some(l), Some(r)) => (l.line_break() + r.line_break(), l.len() + r.len(), l),
            (Some(l), None) => (l.line_break(), l.len(), l),
            (None, Some(r)) => (r.line_break(), r.len(), r),
            (None, Node) => unreachable!(),
        };
        
        return Metadata {
            n_line_break,
            n_char,
            r_depth: node.r_depth() + 1,
        };
    }
}