use std::fmt::Display;

use crate::shared::errors::MalformedSequence;

#[derive(Debug, PartialEq, Eq)]
pub struct SequenceValueObject {
    value: String,
}

impl SequenceValueObject {
    fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl Display for SequenceValueObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

pub trait SequenceValue {
    fn sequence_allowed_chars() -> [char; 4];
}

trait CharsValidEnsurer {
    fn invalid_chars(chars: Vec<char>, target: &str) -> Vec<char> {
        target.chars().filter(|c| !chars.contains(c)).collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RnaSequenceValueObject(SequenceValueObject);

impl RnaSequenceValueObject {
    pub fn new(value: &str) -> Result<Self, MalformedSequence> {
        let self_allowed_characters: [char; 4] = Self::sequence_allowed_chars();
        let invalid_chars = Self::invalid_chars(self_allowed_characters.to_vec(), value);
        if invalid_chars.is_empty() {
            Ok(Self(SequenceValueObject::new(value)))
        } else {
            Err(MalformedSequence::new(
                invalid_chars,
                super::SequenceType::Rna,
            ))
        }
    }
}

impl Display for RnaSequenceValueObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SequenceValue for RnaSequenceValueObject {
    fn sequence_allowed_chars() -> [char; 4] {
        ['a', 'u', 'g', 'c']
    }
}

impl CharsValidEnsurer for RnaSequenceValueObject {}

#[derive(Debug, PartialEq, Eq)]
pub struct DnaSequenceValueObject(SequenceValueObject);

impl Display for DnaSequenceValueObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl DnaSequenceValueObject {
    pub fn new(value: &str) -> Result<Self, MalformedSequence> {
        let self_allowed_characters: [char; 4] = Self::sequence_allowed_chars();
        let invalid_chars = Self::invalid_chars(self_allowed_characters.to_vec(), value);
        if invalid_chars.is_empty() {
            Ok(Self(SequenceValueObject::new(value)))
        } else {
            Err(MalformedSequence::new(
                invalid_chars,
                super::SequenceType::Dna,
            ))
        }
    }
}

impl From<DnaSequenceValueObject> for RnaSequenceValueObject {
    fn from(value: DnaSequenceValueObject) -> Self {
        Self(SequenceValueObject::new(
            &value.0.to_string().replace('t', "u"),
        ))
    }
}

impl From<RnaSequenceValueObject> for DnaSequenceValueObject {
    fn from(value: RnaSequenceValueObject) -> Self {
        Self(SequenceValueObject::new(
            &value.0.to_string().replace('u', "t"),
        ))
    }
}

impl SequenceValue for DnaSequenceValueObject {
    fn sequence_allowed_chars() -> [char; 4] {
        ['a', 't', 'g', 'c']
    }
}

impl CharsValidEnsurer for DnaSequenceValueObject {}

#[cfg(test)]
mod tests {
    use crate::{
        domain::{
            value_objects::{DnaSequenceValueObject, RnaSequenceValueObject},
            SequenceType,
        },
        shared::errors::MalformedSequence,
    };

    #[test]
    fn should_create_rna_sequence_value_object() {
        assert!(RnaSequenceValueObject::new("aucg").is_ok())
    }

    #[test]
    fn should_create_dna_sequence_value_object() {
        assert!(DnaSequenceValueObject::new("atcg").is_ok())
    }

    #[test]
    fn should_error_rna_sequence_value_object() {
        let error_rna: Result<RnaSequenceValueObject, MalformedSequence> =
            RnaSequenceValueObject::new("atcg");
        assert!(error_rna
            .as_ref()
            .is_err_and(|e| e.illegal_chars == vec!['t']));
        assert!(error_rna
            .as_ref()
            .is_err_and(|e| e.kind == SequenceType::Rna));
        assert!(error_rna.as_ref().is_err_and(
            |e| e.to_string() == format!("Illegal characters: 't' are not allowed in rna")
        ))
    }

    #[test]
    fn should_error_dna_sequence_value_object() {
        let error_dna: Result<DnaSequenceValueObject, MalformedSequence> =
            DnaSequenceValueObject::new("aucg");
        println!(
            "{:?}, {:?}",
            error_dna.as_ref().unwrap_err().illegal_chars,
            error_dna.as_ref().unwrap_err().kind
        );
        assert!(error_dna
            .as_ref()
            .is_err_and(|e| e.illegal_chars == vec!['u']));
        assert!(error_dna
            .as_ref()
            .is_err_and(|e| e.kind == SequenceType::Dna));
        assert!(error_dna.as_ref().is_err_and(
            |e| e.to_string() == format!("Illegal characters: 'u' are not allowed in dna")
        ))
    }
}
