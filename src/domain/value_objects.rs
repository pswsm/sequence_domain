use crate::ObjectMother;

use self::errors::SequenceError;

#[derive(Debug, PartialEq, Eq)]
pub struct SequenceValueObject {
    value: String,
}

impl SequenceValueObject {
    pub fn new<T: ToString>(value: T) -> Result<Self, SequenceError> {
        let bind: String = value.to_string();
        match &bind.chars().all(|c| ['a', 't', 'c', 'g', 'u'].contains(&c)) {
            true => Ok(Self { value: bind }),
            false => Err(SequenceError::Malformed(format!("WAWAWA or sum"))),
        }
    }
}

pub(crate) struct SequenceValueObjectMother {
    pub child: SequenceValueObject,
}

impl ObjectMother for SequenceValueObjectMother {
    fn build() -> Self {
        Self {
            child: SequenceValueObject {
                value: "atcg".to_owned(),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::domain::value_objects::{
        ObjectMother, SequenceValueObject, SequenceValueObjectMother,
    };

    macro_rules! setup {
        ($objectMother:ident) => {
            let $objectMother = SequenceValueObjectMother::build();
        };
    }

    #[test]
    fn should_create_seqvobj() {
        assert!(SequenceValueObject::new("atcg").is_ok())
    }

    #[test]
    fn wawa() {
        setup!(vobj);
        assert_eq!(
            vobj.child,
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
