use turbo_crates_testing_proc_macros::*;

fn main() {}

#[safe_tests]
pub(crate) mod test {
    const TEST_ENV_VAR: &str = "TEST_ENV_VAR";

    #[safe_test(a, b)]
    fn it_works1() {
        std::env::set_var(TEST_ENV_VAR, "test1");
        let test_value = std::env::var(TEST_ENV_VAR).unwrap();
        assert_eq!(test_value, "test1");
    }
}
