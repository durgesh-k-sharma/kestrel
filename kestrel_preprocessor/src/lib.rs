//! C preprocessor — handles directives, macro expansion, and file inclusion.
//!
//! Part of the Kestrel C11 compiler project.

/// Returns the crate name.
pub const fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
