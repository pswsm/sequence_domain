use crate::shared::errors::MalformedSequence;

use self::value_objects::{DnaSequenceValueObject, RnaSequenceValueObject, SequenceValueTrait};

pub(crate) mod value_objects;

#[derive(Debug, PartialEq, Eq)]
pub enum SequenceType {
    Dna,
    Rna,
}

pub struct Sequence<T>
where
    T: SequenceValueTrait,
{
    sequence: T,
}

impl Sequence<DnaSequenceValueObject> {
    pub fn new(sequence: &str) -> Result<Self, MalformedSequence> {
        Ok(Sequence {
            sequence: DnaSequenceValueObject::new(sequence)?,
        })
    }
    #[must_use]
    pub fn sequence(&self) -> &DnaSequenceValueObject {
        &self.sequence
    }
}

impl Sequence<RnaSequenceValueObject> {
    pub fn new(sequence: &str) -> Result<Self, MalformedSequence> {
        Ok(Sequence {
            sequence: RnaSequenceValueObject::new(sequence)?,
        })
    }
    #[must_use]
    pub fn sequence(&self) -> &RnaSequenceValueObject {
        &self.sequence
    }
}

#[cfg(test)]
mod tests {
    mod seq {
        use crate::{
            domain::{value_objects::DnaSequenceValueObject, Sequence},
            setup_mother_child,
            shared::object_mothers::{DnaSequenceObjectMother, ObjectMother},
        };

        #[test]
        fn should_create_new() {
            setup_mother_child!(
                common_mother,
                _common_child,
                DnaSequenceObjectMother,
                Sequence<DnaSequenceValueObject>
            );
            let manual_sequence: Sequence<DnaSequenceValueObject> =
                Sequence::<DnaSequenceValueObject>::new("atcg").unwrap();
            assert_eq!(
                &manual_sequence.sequence().to_string(),
                &common_mother.sequence
            );
        }
        #[test]
        fn should_return_sequence() {
            setup_mother_child!(
                common_mother,
                common_child,
                DnaSequenceObjectMother,
                Sequence<DnaSequenceValueObject>
            );
            assert_eq!(
                &common_child.sequence().to_string(),
                &common_mother.sequence
            );
        }
    }
}
