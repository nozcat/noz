# Development process
- Always at the end of a task, build (`cargo build`), run all tests (`cargo test`), format (`cargo fmt`), and lint (`cargo clippy`) to ensure no errors. Also run `cargo tarpaulin` to collect code coverage and ensure the files you are modifying continue to have full code coverage.
- Unit tests should live in a separate `test/` folder and module under `src/`, not in the implementation file itself.
