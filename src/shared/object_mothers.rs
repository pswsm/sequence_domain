use crate::domain::{
    value_objects::{DnaSequenceValueObject, RnaSequenceValueObject},
    Sequence,
};

pub trait ObjectMother {
    type Item;
    fn build(self) -> Self::Item;
    fn init() -> Self;
}

pub struct DnaSequenceObjectMother {
    pub sequence: String,
}

impl DnaSequenceObjectMother {
    fn with_sequence(self, sequence: &str) -> Self {
        Self {
            sequence: sequence.to_string(),
        }
    }
}

impl ObjectMother for DnaSequenceObjectMother {
    type Item = Sequence<DnaSequenceValueObject>;
    fn build(self) -> Self::Item {
        Self::Item::new(&self.sequence).unwrap()
    }
    fn init() -> Self {
        Self {
            sequence: "atcg".to_string(),
        }
    }
}

pub struct RnaSequenceObjectMother {
    pub sequence: String,
}

impl RnaSequenceObjectMother {
    fn with_sequence(self, sequence: &str) -> Self {
        Self {
            sequence: sequence.to_string(),
        }
    }
}

impl ObjectMother for RnaSequenceObjectMother {
    type Item = Sequence<RnaSequenceValueObject>;
    fn build(self) -> Self::Item {
        Self::Item::new(&self.sequence).unwrap()
    }
    fn init() -> Self {
        Self {
            sequence: "aucg".to_string(),
        }
    }
}
