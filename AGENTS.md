# Hurl Agent Guidelines

## Build & Test Commands
- Build: `cargo build`
- Run tests: `cargo test --lib`
- Run integration tests: `cd integration/hurl && ./integration.py`
- Run single test: `cargo test <test_name>` or `cargo test --package hurl_core <test_name>`
- Lint: `cargo clippy --all-targets`

## Code Style
- Use `rustfmt` for formatting
- No wildcard imports (`use some::*` is denied)
- Clear, concise commit messages starting with capital letter (e.g., "Fix bug" not "fix: bug")
- Signed commits required
- Error handling: use Result/Option types, minimal dependencies, panic rarely
- Naming: follow Rust conventions (snake_case for variables/functions, CamelCase for types)
- Documentation: all public APIs should be documented
- Linear Git history, avoid merge commits
- Avoid external dependencies where possible
- Follow Hurl core values: CLI-first, text-based format, multiplatform
- All tests must pass before merge

## Project Structure
- Main crates: hurl, hurlfmt, hurl_core
- Run integration tests with Python 3.9+ in virtual environment