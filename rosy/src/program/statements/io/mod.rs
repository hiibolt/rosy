//! # I/O Statements
//!
//! File and console input/output operations.
//!
//! | Module | Statement | Description |
//! |--------|-----------|-------------|
//! | [`mod@write`] | `WRITE unit exprs;` | Write formatted text |
//! | [`read`] | `READ unit var;` | Read a value from a unit |
//! | [`writeb`] | `WRITEB unit exprs;` | Write binary data |
//! | [`readb`] | `READB unit var;` | Read binary data |
//! | [`openf`] | `OPENF unit file status;` | Open a text file |
//! | [`openfb`] | `OPENFB unit file status;` | Open a binary file |
//! | [`closef`] | `CLOSEF unit;` | Close a file unit |
//!
//! ## Unit Numbers
//!
//! Unit `6` is standard output (console). Other unit numbers map to
//! files opened with `OPENF` / `OPENFB`.

pub mod closef;
pub mod cpusec;
pub mod openf;
pub mod openfb;
pub mod os_call;
pub mod read;
pub mod readb;
pub mod velget;
pub mod write;
pub mod writeb;