//! # Operators
//!
//! Binary and unary operator expressions. Each operator uses a registry-driven
//! type system defined in [`rosy_lib::operators`](crate::rosy_lib::operators)
//! that serves as the single source of truth for type compatibility.
//!
//! ## Binary Operators
//!
//! | Operator | Symbol | Module | Description |
//! |----------|--------|--------|-------------|
//! | Addition | `+` | [`add`] | Arithmetic addition, logical OR |
//! | Subtraction | `-` | [`sub`] | Arithmetic subtraction |
//! | Multiplication | `*` | [`mult`] | Arithmetic multiplication, logical AND |
//! | Division | `/` | [`div`] | Arithmetic division |
//! | Power | `^` | (in [`functions::math::pow`](super::functions::math::pow)) | Exponentiation |
//! | Concatenation | `&` | [`mod@concat`] | Vector/string concatenation |
//! | Extraction | `\|` | [`extract`] | Element/substring extraction |
//! | Derivation | `%` | [`mod@derive`] | DA partial derivative |
//! | Equal | `=` | [`eq`] | Equality comparison |
//! | Not Equal | `<>` | [`neq`] | Inequality comparison |
//! | Less Than | `<` | [`lt`] | Numeric/lexicographic less-than |
//! | Greater Than | `>` | [`gt`] | Numeric/lexicographic greater-than |
//! | Less or Equal | `<=` | [`lte`] | Less-than-or-equal |
//! | Greater or Equal | `>=` | [`gte`] | Greater-than-or-equal |
//!
//! ## Unary Operators
//!
//! | Operator | Symbol | Module | Description |
//! |----------|--------|--------|-------------|
//! | Negation | `-` | [`neg`] | Unary minus (transpiled as `0 - x`) |
//! | Logical NOT | `NOT` | [`not`] | Logical negation |

pub mod add;
pub mod concat;
pub mod derive;
pub mod div;
pub mod eq;
pub mod extract;
pub mod gt;
pub mod gte;
pub mod lt;
pub mod lte;
pub mod mult;
pub mod neg;
pub mod neq;
pub mod not;
pub mod sub;