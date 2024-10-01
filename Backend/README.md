> NOTE: To use watch, install bacon with `cargo install bacon`.

## Dev (watch)

```sh
# Terminal 1 - To run the server.
bacon run-backend

# Terminal 2 - To run the quick_dev (tests).
bacon run-quick_dev
```

## Dev (cargo)

```sh
# Terminal 1 - To run the server.
cargo run --color always

# Terminal 2 - To run the quick_dev (tests).
cargo run --example quick_dev --color always
```

## Unit Test (watch)

```sh
bacon test

# Specific test with filter.
bacon test -- model::task::tests::test_create
```

## Unit Test (cargo)

```sh
cargo test -- --nocapture --color always

# Specific test with filter.
cargo test -- model::task::tests::test_create --nocapture --color always
```
