//! # Memory Estimation Functions (COSY Compatibility)
//!
//! These functions exist for COSY INFINITY compatibility. In Rosy, memory
//! management is handled automatically, so these always return `0`.
//!
//! | Function | Description |
//! |----------|-------------|
//! | `LST(x)` | String memory estimate |
//! | `LCM(x)` | Complex memory estimate |
//! | `LCD(x)` | DA memory estimate |

pub mod lst;
pub mod lcm;
pub mod lcd;
