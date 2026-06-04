# Contributing to Kestrel

Thank you for your interest in contributing to Kestrel!

## Getting Started

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `cargo test --workspace`
5. Run clippy: `cargo clippy --workspace --all-targets -- -D warnings`
6. Format: `cargo fmt --all`
7. Submit a pull request

## Code Standards

- All code must pass `cargo clippy -- -D warnings`
- All code must be formatted with `cargo fmt`
- All public items must have documentation comments
- All new features must include tests

## License

By contributing, you agree that your contributions will be licensed under
the same dual MIT/Apache-2.0 license as the project.
