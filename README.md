# Kestrel

[![CI](https://github.com/durgesh-k-sharma/kestrel/actions/workflows/ci.yml/badge.svg)](https://github.com/durgesh-k-sharma/kestrel/actions/workflows/ci.yml)
[![Docs](https://github.com/durgesh-k-sharma/kestrel/actions/workflows/docs.yml/badge.svg)](https://github.com/durgesh-k-sharma/kestrel/actions/workflows/docs.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![crates.io](https://img.shields.io/crates/v/kestrel.svg)](https://crates.io/crates/kestrel)

**Kestrel** is a C11 compiler written in Rust. It features:

- **Multi-backend**: x86-64, ARM64 (AArch64), RISC-V
- **SSA-based IR** with 11 optimization passes
- **Integrated C preprocessor** (no external cpp needed)
- **Library + CLI** — use as a Rust library or command-line tool
- **Comprehensive testing** — unit, integration, differential, fuzzing

## Status

🚧 Active development — [M0: Scaffold](docs/plans/2026-06-04-m0-scaffold.md)

## Quick Start

```bash
# Build
cargo build --release --bin kestrel

# Run
cargo run --bin kestrel

# Test
cargo test --workspace
```

## Architecture

```
Source → Preprocessor → Lexer → Parser → Semantic Analysis → AST → SSA IR → Optimizations → Backend
```

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
