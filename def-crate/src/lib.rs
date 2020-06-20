#[macro_export]
macro_rules! define_macro {
    () => {
        #[macro_export]
        macro_rules! constructed_macro {
            () => {
                $crate::IN_DEF_CRATE;
            }
        }
    }
}
