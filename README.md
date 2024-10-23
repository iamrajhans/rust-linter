# Code Linter

A simple code linter for Rust that detects common issues like unused variables and improper naming conventions.

## Features
- Detects unused variables.
- Ensures functions follow snake_case naming conventions.
- Warns about high cyclomatic complexity in loops and nested if statements.
- Identifies dead code (unused functions).
- Provides JSON output for easy integration with CI/CD tools.
- Automatically fixes some linting issues using `rustfmt`.
- Uses multi-threading to speed up linting for larger files.

## How to Run

1. Clone the repository:
   ```
   git clone <repository_url>
   ```

2. Build and run the project:
   ```
   cargo run -- <file_to_lint.rs>
   ```

3. Lint a Rust file by specifying its path as a command-line argument.

4. To specify the output format (plain or JSON):
   ```
   cargo run -- <file_to_lint.rs> -o json
   ```

5. To automatically fix formatting issues using `rustfmt`:
   ```
   cargo run -- <file_to_lint.rs> --fix
   ```
