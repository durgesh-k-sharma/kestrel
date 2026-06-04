//! Semantic analysis and type checking for C11.
//!
//! Part of the Kestrel C11 compiler project.

/// Returns the crate name.
pub const fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
