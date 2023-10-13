macro_rules! async_test {
    ($func_name:tt, $code_block:block) => {
        #[tokio::test]
        async fn $func_name() -> anyhow::Result<()> {
            env_logger::init();
            $code_block
            Ok(())
        }
    };
}

macro_rules! test {
    ($func_name:tt, $code_block:block) => {
        #[test]
        fn $func_name() -> anyhow::Result<()> {
            env_logger::init();
            $code_block
            Ok(())
        }
    };
}
