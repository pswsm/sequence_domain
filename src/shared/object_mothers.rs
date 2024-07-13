use crate::domain::{
    value_objects::{DnaSequenceValueObject, RnaSequenceValueObject},
    Sequence,
};

pub trait SequenceMother {
    type Item;
    fn build(self) -> Self::Item;
    fn init() -> Self;
    fn with_sequence(self, new_value: &str) -> Self;
}

pub struct DnaSequenceObjectMother {
    pub sequence: String,
}

impl SequenceMother for DnaSequenceObjectMother {
    type Item = Sequence<DnaSequenceValueObject>;
    fn build(self) -> Self::Item {
        Self::Item::new(&self.sequence).unwrap()
    }
    fn init() -> Self {
        Self {
            sequence: "atcg".to_string(),
        }
    }
    fn with_sequence(mut self, new_value: &str) -> Self {
        self.sequence = new_value.to_string();
        self
    }
}

pub struct RnaSequenceObjectMother {
    pub sequence: String,
}

impl SequenceMother for RnaSequenceObjectMother {
    type Item = Sequence<RnaSequenceValueObject>;
    fn build(self) -> Self::Item {
        Self::Item::new(&self.sequence).unwrap()
    }
    fn init() -> Self {
        Self {
            sequence: "aucg".to_string(),
        }
    }
    fn with_sequence(mut self, new_value: &str) -> Self {
        self.sequence = new_value.to_string();
        self
    }
}
