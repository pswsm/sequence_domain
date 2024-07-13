use std::fmt::Display;

use crate::domain::SequenceType;

#[derive(Debug)]
pub struct MalformedSequence {
    pub illegal_chars: Vec<String>,
    pub kind: SequenceType,
}

impl MalformedSequence {
    pub fn new(illegal_chars: &[String], kind: SequenceType) -> Self {
        Self {
            illegal_chars: illegal_chars
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
            kind,
        }
    }
}

impl Display for MalformedSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Illegal characters: '{}' are not allowed in {}",
            self.illegal_chars.join(", "),
            self.kind
        )
    }
}

impl std::error::Error for MalformedSequence {}
