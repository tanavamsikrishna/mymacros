macro_rules! new_deserializer {
    ($func_name:ident $to_type:ty {$exec_block:expr}) => {
        pub(crate) fn $func_name<'de, D>(d: D) -> std::result::Result<$to_type, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let s = Deserialize::deserialize(d)?;
            $exec_block(s).map_err(de::Error::custom)
        }
    };
}

macro_rules! new_str_serializer {
    ($func_name:ident $from_type:ty {$exec_block:expr}) => {
        fn $func_name<S>(from_val: &$from_type, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            s.serialize_str(&$exec_block(&from_val))
        }
    };
}
