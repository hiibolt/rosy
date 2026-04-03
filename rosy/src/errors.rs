//! # Structured Errors
//!
//! Error types that carry source location information for precise diagnostics.
//! These integrate with `anyhow` — they implement `std::error::Error` and can
//! be recovered from an `anyhow::Error` chain via `error.downcast_ref::<RosyError>()`.

use crate::program::statements::SourceLocation;

/// A ROSY error with an associated source location.
///
/// Use this instead of plain `anyhow!()` whenever a `SourceLocation` is available.
/// The LSP extracts the location via `downcast_ref` to place diagnostics precisely.
#[derive(Debug)]
pub struct RosyError {
    /// Human-readable error message.
    pub message: String,
    /// Where in the source the error occurred.
    pub location: Option<SourceLocation>,
    /// The severity of the error.
    pub severity: RosyErrorSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RosyErrorSeverity {
    Error,
    Warning,
}

impl RosyError {
    /// Create an error at a known source location.
    pub fn at(location: SourceLocation, message: impl Into<String>) -> Self {
        RosyError {
            message: message.into(),
            location: Some(location),
            severity: RosyErrorSeverity::Error,
        }
    }

    /// Create an error without a known location.
    pub fn unlocated(message: impl Into<String>) -> Self {
        RosyError {
            message: message.into(),
            location: None,
            severity: RosyErrorSeverity::Error,
        }
    }

    /// Create a warning at a known source location.
    pub fn warning_at(location: SourceLocation, message: impl Into<String>) -> Self {
        RosyError {
            message: message.into(),
            location: Some(location),
            severity: RosyErrorSeverity::Warning,
        }
    }
}

impl std::fmt::Display for RosyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(loc) = &self.location {
            write!(f, "{}: {}", loc, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for RosyError {}
