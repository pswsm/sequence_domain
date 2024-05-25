pub mod errors;
pub mod value_objects;

pub mod test_utils {
    #[macro_export]
    macro_rules! setup {
        ($motherVariable:ident, $childVariable:ident, $motherType:ty, $childType:ty) => {
            let $motherVariable: $motherType = <$motherType>::init();
            let $childVariable: $childType = <$motherType>::init().build();
        };
    }
}
