pub mod bool_qs {
    use serde::Serializer;

    pub fn is_false(b: &bool) -> bool {
        !*b
    }
    pub fn is_true(b: &bool) -> bool {
        *b
    }

    pub fn serialize<S: Serializer>(b: &bool, s: S) -> Result<S::Ok, S::Error> {
        if *b {
            s.serialize_i64(1)
        } else {
            s.serialize_i64(0)
        }
    }
}
