#[macro_export]
macro_rules! imc {
    ($variable_name:ident, $lambda:expr) => {
        $variable_name.iter().map($lambda).collect()
    };
}
