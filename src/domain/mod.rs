use crate::shared::errors::errors::SequenceError;

use self::value_objects::SequenceValueObject;

pub(crate) mod value_objects;

#[derive(Debug, PartialEq, Eq)]
pub enum SequenceType {
    Dna,
    Rna,
}

pub struct Sequence {
    variant: SequenceType,
    sequence: SequenceValueObject,
}

impl Sequence {
    pub fn new<T: ToString>(variant: SequenceType, sequence: T) -> Result<Self, SequenceError> {
        Ok(Sequence {
            variant,
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
    mod seq {
        use crate::{
            domain::{Sequence, SequenceType},
            setup,
            shared::value_objects::{ObjectMother, SequenceObjectMother},
        };

        #[test]
        fn should_create_new() {
            setup!(common_mother, _common_child, SequenceObjectMother, Sequence);
            let manual_sequence: Sequence = Sequence::new(SequenceType::Dna, "atcg").unwrap();

            assert_eq!(manual_sequence.variant(), &common_mother.variant);
            assert_eq!(manual_sequence.sequence(), &common_mother.sequence);
        }

        #[test]
        fn should_return_variant() {
            setup!(common_mother, common_child, SequenceObjectMother, Sequence);
            assert_eq!(common_child.variant(), &common_mother.variant);
        }

        #[test]
        fn should_return_sequence() {
            setup!(common_mother, common_child, SequenceObjectMother, Sequence);
            assert_eq!(common_child.sequence(), &common_mother.sequence);
        }
    }
}
