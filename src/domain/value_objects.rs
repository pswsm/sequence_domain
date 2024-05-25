use std::fmt::Display;

use crate::shared::errors::errors::SequenceError;

#[derive(Debug, PartialEq, Eq)]
pub struct SequenceValueObject {
    value: String,
}

impl SequenceValueObject {
    pub fn new<T: ToString>(value: T) -> Result<Self, SequenceError> {
        let bind: String = value.to_string();
        match &bind.chars().all(|c| ['a', 't', 'c', 'g', 'u'].contains(&c)) {
            true => Ok(Self { value: bind }),
            false => Err(SequenceError::Malformed("WAWAWA or sum".to_string())),
        }
    }
}

impl Display for SequenceValueObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        domain::value_objects::SequenceValueObject,
        setup,
        shared::value_objects::{ObjectMother, SequenceValueObjectMother},
    };

    #[test]
    fn should_create_seqvobj() {
        assert!(SequenceValueObject::new("atcg").is_ok())
    }

    #[test]
    fn wawa() {
        setup!(
            _common_mother,
            common_child,
            SequenceValueObjectMother,
            SequenceValueObject
        );
        assert_eq!(
            common_child,
            SequenceValueObject {
                value: "atcg".to_owned()
            }
        )
    }

    #[test]
    fn should_return_malformed_error() {
        assert!(SequenceValueObject::new("jnhgfkj").is_err())
    }
}
