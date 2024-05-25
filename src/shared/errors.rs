pub mod errors {
    use std::fmt::Display;

    #[derive(Debug)]
    pub enum SequenceError {
        Malformed(String),
    }

    impl Display for SequenceError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "dlkgflksm")
        }
    }

    impl std::error::Error for SequenceError {}
}
