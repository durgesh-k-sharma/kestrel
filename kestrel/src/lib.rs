//! Kestrel — A C11 compiler written in Rust.
//!
//! This crate re-exports all Kestrel compiler library crates.

pub use kestrel_common;
pub use kestrel_lexer;
pub use kestrel_preprocessor;
pub use kestrel_parser;
pub use kestrel_sema;
pub use kestrel_ir;
pub use kestrel_opt;
pub use kestrel_codegen;
