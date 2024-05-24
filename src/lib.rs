pub mod domain;

pub(crate) trait ObjectMother {
    fn build() -> Self;
}

#[cfg(test)]
mod libs {
    use self::domain::{Sequence, SequenceType};

    use super::*;

    fn setup() -> Sequence {
        Sequence::new(Some(SequenceType::Rna), "atcg").unwrap()
    }

    #[test]
    fn it_works() {
        let setup = setup();
        assert_eq!(setup.variant(), &SequenceType::Rna);
    }
}
