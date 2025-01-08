## Prerequisites
- Rust (rustc 1.81.0, cargo 1.81.0)
- PostgreSQL 17.0

## Start the server

```sh
cargo run
```

## Perform tests

```sh
# Perform all unit tests
cargo test

# Perform specific unit tests, replace the test module with the module you want to test.
cargo test -- model::user::tests::test_create
```
