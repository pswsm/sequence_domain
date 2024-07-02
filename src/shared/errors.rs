use std::fmt::Display;

#[derive(Debug)]
pub struct MalformedSequence {
    message: String,
}

impl MalformedSequence {
    pub fn new<T>(message: &T) -> Self
    where
        T: ToString + ?Sized,
    {
        Self {
            message: message.to_string(),
        }
    }
}

impl Display for MalformedSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl std::error::Error for MalformedSequence {}
