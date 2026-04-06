#![cfg_attr(feature = "nightly-simd", feature(portable_simd))]
//! # Rosy
//!
#![doc = concat!("**Version:** `v", env!("CARGO_PKG_VERSION"), "` — Built `", env!("BUILD_TIMESTAMP"), "` — [Changelog](https://github.com/hiibolt/rosy/releases)")]
//!
//! A modern transpiler for the Rosy scientific programming language,
//! designed for beam physics and differential algebra applications.
//! Rosy programs are transpiled into self-contained, native Rust executables.
//!
//! ## Language Reference
//! The official Rosy language reference begins in the [`program`] module.
//!
//! ## More Resources
//! - **[Example programs](https://github.com/hiibolt/rosy/tree/master/examples)** on GitHub
//! - **[Installation & usage](https://github.com/hiibolt/rosy)** in the README

pub mod ast;
pub mod embedded;
pub mod errors;
pub mod lsp;
pub mod preprocess;
pub mod program;
pub mod resolve;
#[allow(unused_imports, dead_code)]
pub mod rosy_lib;
pub mod syntax_config;
pub mod transpile;
