# Development Process

## Task Completion Checklist

After completing any task, always perform the following steps:

1. **Build the project:**
   ```sh
   cargo build
   ```
   - There should be no errors or warnings.
2. **Run all tests:**
   ```sh
   cargo test
   ```
   - There should be no errors or warnings and all tests should pass.
3. **Format the code:**
   ```sh
   cargo fmt
   ```
   - There should be no errors or warnings.
4. **Lint the code:**
   ```sh
   cargo clippy
   ```
   - There should be no errors or warnings.
5. **Check code coverage:**
   ```sh
   cargo tarpaulin
   ```
   - Ensure that all files you modify continue to have full test coverage.
6. **Review your changes:**
   - Diff your changes against the previous commit.
   - Act as a linter and find issues or improvements.
   - Address those issues or improvements you notice.

## Commit Messages

- After every commit, print the full commit message in the terminal.

# Code Organization

## File Size

- Files should generally not exceed 500 lines. Only exceed this limit if breaking up the file would make the code more confusing or harder to maintain.

## Unit Tests

- Unit tests must reside in a separate `test/` folder and module under `src/`.
- Do **not** place unit tests in the implementation file itself.
- A single source file may have its tests split across multiple test modules in different files within the `test/` directory, for better organization and clarity, especially for large or complex modules.
