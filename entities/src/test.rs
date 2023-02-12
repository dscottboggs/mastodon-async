#[macro_export(crate)]
macro_rules! serde_value_test {
    ($fn_name:ident($ty:ty): $example:literal) => {
        #[test]
        fn $fn_name() {
            let subject: $ty = serde_json::from_str($example).expect(concat!(
                "example for ",
                stringify!($ty),
                " failed to deserialize"
            ));
            let as_value: serde_json::Value = serde_json::from_str($example).expect(concat!(
                "example for ",
                stringify!($ty),
                " couldn't be parsed as Value"
            ));
            let subject_as_value =
                serde_json::to_value(subject).expect("failed to convert to value");
            assert_eq!(as_value, subject_as_value);
        }
    };
}
