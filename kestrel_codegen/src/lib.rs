//! Backend trait and code generation for x86-64, ARM64, and RISC-V.
//!
//! Part of the Kestrel C11 compiler project.

/// Returns the crate name.
pub const fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
