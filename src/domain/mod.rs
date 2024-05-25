use self::value_objects::{errors::SequenceError, SequenceValueObject};

#[derive(Debug, PartialEq, Eq)]
pub enum SequenceType {
    Dna,
    Rna,
}

mod value_objects;

pub struct Sequence {
    variant: SequenceType,
    sequence: SequenceValueObject,
}

impl Sequence {
    pub fn new<T: ToString>(
        variant: Option<SequenceType>,
        sequence: T,
    ) -> Result<Self, SequenceError> {
        Ok(Sequence {
            variant: variant.unwrap_or(SequenceType::Dna),
            sequence: SequenceValueObject::new(sequence)?,
        })
    }
    pub fn variant(&self) -> &SequenceType {
        &self.variant
    }
    pub fn sequence(&self) -> &SequenceValueObject {
        &self.sequence
    }
}

#[cfg(test)]
mod tests {
    use crate::ObjectMother;
    use crate::domain::value_objects::{SequenceValueObjectMother, SequenceValueObject}

    mod seq {
        use crate::domain::{Sequence, SequenceType};

        #[test]
        fn should_create_new() {
            setup!(test_value_object, test_sequence);
            let built_sequence = Sequence::new(None, "atcg".to_string()).unwrap();

            assert_eq!(&built_sequence.variant(), &test_sequence.variant());
            assert_eq!(&built_sequence.sequence(), &test_sequence.sequence());
        }

        #[test]
        fn should_return_variant() {
            setup!(test_value_object, test_sequence);
            assert_eq!(test_sequence.variant(), &SequenceType::Dna);
        }

        #[test]
        fn should_return_sequence() {
            setup!(test_value_object, test_sequence);
            assert_eq!(test_sequence.sequence(), &test_value_object);
        }
    }
}
