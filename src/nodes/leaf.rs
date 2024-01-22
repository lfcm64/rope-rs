use super::metadata::Metadata;
use std::fmt;

#[derive(Clone)]
pub struct Leaf {
    pub str: String,
    pub metadata: Metadata,
}

impl Leaf {
    pub fn new(str: &str) -> Self {
        let n_char = str.len();
        let n_break_line = str.chars().filter(|&c| c == '\n').count();

        let metadata = Metadata::new(n_break_line, n_char, 0);

        Leaf {
            str: str.to_string(),
            metadata,
        }
    }

    pub fn line(&self, index: usize) -> Option<String> {
        let lines: Vec<&str> = self.str.lines().collect();
        if index < lines.len() {
            return Some(lines[index].to_string())
        } 
        None
    }
}

impl fmt::Display for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}