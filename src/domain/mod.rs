use std::fmt::Display;

use crate::shared::errors::MalformedSequence;

use self::value_objects::{DnaSequenceValueObject, RnaSequenceValueObject, SequenceValueTrait};

pub(crate) mod value_objects;

#[derive(Debug, PartialEq, Eq)]
pub enum SequenceType {
    Dna,
    Rna,
}

impl Display for SequenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dna => write!(f, "dna"),
            Self::Rna => write!(f, "rna"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Sequence<T>
where
    T: SequenceValueTrait,
{
    sequence: T,
}

impl Sequence<DnaSequenceValueObject> {
    /// Try creating a new sequence, or return error
    /// # Errors
    /// - `MalformedSequence`:
    ///      The sequence has invalid characters
    pub fn new(sequence: &str) -> Result<Self, MalformedSequence> {
        Ok(Sequence {
            sequence: DnaSequenceValueObject::new(sequence)?,
        })
    }

    pub fn get_sequence(&self) -> &DnaSequenceValueObject {
        &self.sequence
    }

    pub fn to_rna(&self) -> Sequence<RnaSequenceValueObject> {
        Sequence {
            sequence: RnaSequenceValueObject::new(
                &self.get_sequence().to_string().replace("t", "u"),
            )
            .unwrap(),
        }
    }
}

impl Sequence<RnaSequenceValueObject> {
    /// Try creating a new sequence, or return error
    /// # Errors
    /// - `MalformedSequence`:
    ///      The sequence has invalid characters
    pub fn new(sequence: &str) -> Result<Self, MalformedSequence> {
        Ok(Sequence {
            sequence: RnaSequenceValueObject::new(sequence)?,
        })
    }
    #[must_use]
    pub fn get_sequence(&self) -> &RnaSequenceValueObject {
        &self.sequence
    }
    pub fn to_dna(&self) -> Sequence<DnaSequenceValueObject> {
        Sequence {
            sequence: DnaSequenceValueObject::new(
                &self.get_sequence().to_string().replace("u", "t"),
            )
            .unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    mod seq {
        use crate::{
            domain::{
                value_objects::{DnaSequenceValueObject, RnaSequenceValueObject},
                Sequence,
            },
            setup_mother_child,
            shared::object_mothers::{
                DnaSequenceObjectMother, RnaSequenceObjectMother, SequenceMother,
            },
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
                &manual_sequence.get_sequence().to_string(),
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
                &common_child.get_sequence().to_string(),
                &common_mother.sequence
            );
        }
        #[test]
        fn should_change_to_rna() {
            let rna: Sequence<RnaSequenceValueObject> = RnaSequenceObjectMother::init()
                .with_sequence("aucg")
                .build();
            let dna: Sequence<DnaSequenceValueObject> = DnaSequenceObjectMother::init()
                .with_sequence("atcg")
                .build();

            assert_eq!(dna.to_rna(), rna)
        }
        #[test]
        fn should_change_to_dna() {
            let rna: Sequence<RnaSequenceValueObject> = RnaSequenceObjectMother::init()
                .with_sequence("aucg")
                .build();
            let dna: Sequence<DnaSequenceValueObject> = DnaSequenceObjectMother::init()
                .with_sequence("atcg")
                .build();

            assert_eq!(rna.to_dna(), dna)
        }
    }
}
