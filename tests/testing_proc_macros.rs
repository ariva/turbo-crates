use turbo_crates::testing::proc_macros::*;

#[cfg(test)]
#[safe_tests]
pub(crate) mod test {
    const TEST_ENV_VAR: &str = "TEST_ENV_VAR";

    #[test]
    #[safe_test]
    fn it_works1() {
        std::env::set_var(TEST_ENV_VAR, "test1");
        let test_value = std::env::var(TEST_ENV_VAR).unwrap();
        assert_eq!(test_value, "test1");
    }
    #[test]
    #[safe_test]
    fn it_works2() {
        std::env::set_var(TEST_ENV_VAR, "test2");
        let test_value = std::env::var(TEST_ENV_VAR).unwrap();
        assert_eq!(test_value, "test2");
    }
    #[test]
    #[safe_test]
    fn it_works3() {
        std::env::set_var(TEST_ENV_VAR, "test3");
        let test_value = std::env::var(TEST_ENV_VAR).unwrap();
        assert_eq!(test_value, "test3");
    }
    #[test]
    #[safe_test]
    fn it_works4() {
        std::env::set_var(TEST_ENV_VAR, "test4");
        let test_value = std::env::var(TEST_ENV_VAR).unwrap();
        assert_eq!(test_value, "test4");
    }
}
