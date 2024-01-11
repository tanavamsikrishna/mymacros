#[macro_export]
macro_rules! async_test {
    ($func_name:tt, $code_block:block) => {
        #[tokio::test]
        async fn $func_name() {
            let _ = env_logger::try_init();
            $code_block;
        }
    };
}

#[macro_export]
macro_rules! test {
    ($func_name:tt, $code_block:block) => {
        #[test]
        fn $func_name() {
            let _ = env_logger::try_init();
            $code_block;
        }
    };
}
