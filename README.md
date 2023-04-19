# turbo-crates

A set of useful crates (WIP)

## Included crates

### `testing_proc_macros` - Used to make testing more predictable

Include:

```toml
turbo-crates = { version="0.0.3", features=["testing_proc_macros"] }
```

```rust
use turbo_crates::testing::proc_macros::*;
```

Use:

```rust
#[cfg(test)]
#[safe_tests]
pub(crate) mod test {
    const TEST_ENV_VAR: &str = "TEST_ENV_VAR";

    #[test]
    #[safe_test]
    fn main_it_works1() {
        std::env::set_var(TEST_ENV_VAR, "test1");
        let test_value = std::env::var(TEST_ENV_VAR).unwrap();
        assert_eq!(test_value, "test1");
    }
    // ...
}
```
