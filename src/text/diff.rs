#![allow(unused)]

use super::Text;

struct TextDiff {
    pub parent_commit: Text,
    pub child_commit: Text,
}

impl TextDiff {
    pub fn new(parent_commit: Text, child_commit: Text) -> Self {
        Self {
            parent_commit,
            child_commit
        }
    }

    
}