//! Operator test registry system.
//! 
//! This module defines the declarative format for operator compatibility testing.
//! The registry is parsed at build-time to generate:
//! - ROSY test scripts
//! - COSY test scripts  
//! - Documentation tables
//! - Integration tests

/// Defines a type compatibility rule for an operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeRule {
    /// Left-hand side type
    pub lhs: &'static str,
    /// Right-hand side type
    pub rhs: &'static str,
    /// Result type
    pub result: &'static str,
    /// Optional comment for documentation
    pub comment: &'static str,
}

impl TypeRule {
    /// Create a new type rule without a comment.
    pub const fn new(lhs: &'static str, rhs: &'static str, result: &'static str) -> Self {
        Self { lhs, rhs, result, comment: "" }
    }
    
    /// Create a new type rule with a comment.
    pub const fn with_comment(
        lhs: &'static str,
        rhs: &'static str,
        result: &'static str,
        comment: &'static str,
    ) -> Self {
        Self { lhs, rhs, result, comment }
    }
}

/// Macro to define operator registries more concisely.
/// 
/// # Example
/// ```ignore
/// operator_registry! {
///     ADD => [
///         RE + RE => RE,
///         RE + CM => CM,
///         CM + DA => CD ("Complex DA result"),
///     ]
/// }
/// ```
#[macro_export]
macro_rules! operator_registry {
    // Entry with comment
    ($lhs:ident + $rhs:ident => $result:ident ($comment:literal)) => {
        $crate::operators::registry::TypeRule::with_comment(
            stringify!($lhs),
            stringify!($rhs),
            stringify!($result),
            $comment
        )
    };
    
    // Entry without comment
    ($lhs:ident + $rhs:ident => $result:ident) => {
        $crate::operators::registry::TypeRule::new(
            stringify!($lhs),
            stringify!($rhs),
            stringify!($result)
        )
    };
}
