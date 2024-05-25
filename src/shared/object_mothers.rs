use crate::domain::{value_objects::SequenceValueObject, Sequence, SequenceType};

pub(crate) trait ObjectMother {
    type Item;
    fn build(self) -> Self::Item;
    fn init() -> Self;
}

pub struct SequenceValueObjectMother {
    pub value: String,
}

impl ObjectMother for SequenceValueObjectMother {
    type Item = SequenceValueObject;
    fn build(self) -> Self::Item {
        SequenceValueObject::new(self.value).unwrap()
    }
    fn init() -> Self {
        Self {
            value: "atcg".to_string(),
        }
    }
}

pub struct SequenceObjectMother {
    pub variant: SequenceType,
    pub sequence: SequenceValueObject,
}

impl ObjectMother for SequenceObjectMother {
    type Item = Sequence;
    fn build(self) -> Self::Item {
        Sequence::new(self.variant, self.sequence).unwrap()
    }
    fn init() -> Self {
        Self {
            variant: SequenceType::Dna,
            sequence: SequenceValueObjectMother::init().build(),
        }
    }
}
