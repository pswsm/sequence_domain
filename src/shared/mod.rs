pub mod errors;
pub mod object_mothers;

pub mod test_utils {
    #[macro_export]
    macro_rules! setup_mother_child {
        ($motherVariable:ident, $childVariable:ident, $motherType:ty, $childType:ty) => {
            let $motherVariable: $motherType = <$motherType>::init();
            let $childVariable: $childType = <$motherType>::init().build();
        };
    }
}
