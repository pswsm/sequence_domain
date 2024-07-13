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

pub trait SequenceValueTrait {
    fn sequence_allowed_chars() -> [char; 4];
}

#[derive(Debug, PartialEq, Eq)]
pub struct RnaSequenceValueObject(SequenceValueObject);

impl RnaSequenceValueObject {
    pub fn new(value: &str) -> Result<Self, MalformedSequence> {
        let self_allowed_characters: [char; 4] = Self::sequence_allowed_chars();
        let illegal_chars: Vec<String> = value
            .chars()
            .filter(|c| !self_allowed_characters.contains(c))
            .map(|c| c.to_string())
            .collect();
        if illegal_chars.is_empty() {
            Ok(Self(SequenceValueObject::new(value)))
        } else {
            Err(MalformedSequence::new(
                &illegal_chars,
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

impl SequenceValueTrait for RnaSequenceValueObject {
    fn sequence_allowed_chars() -> [char; 4] {
        ['a', 'u', 'g', 'c']
    }
}

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
        let illegal_chars: Vec<String> = value
            .chars()
            .filter(|c| !self_allowed_characters.contains(c))
            .map(|c| c.to_string())
            .collect();
        if illegal_chars.is_empty() {
            Ok(Self(SequenceValueObject::new(value)))
        } else {
            Err(MalformedSequence::new(
                &illegal_chars,
                super::SequenceType::Dna,
            ))
        }
    }
}

impl SequenceValueTrait for DnaSequenceValueObject {
    fn sequence_allowed_chars() -> [char; 4] {
        ['a', 't', 'g', 'c']
    }
}

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
        assert!(RnaSequenceValueObject::new("atcg").is_err())
    }

    #[test]
    fn should_error_dna_sequence_value_object() {
        let error_dna: Result<DnaSequenceValueObject, MalformedSequence> =
            DnaSequenceValueObject::new("aucg");
        assert!(error_dna.as_ref().is_err_and(
            |e| e.illegal_chars == vec!["u".to_string()] && e.kind == SequenceType::Dna
        ));
        assert!(
            error_dna.unwrap_err().to_string()
                == format!("Illegal characters: 'u' are not allowed in dna")
        )
    }
}
