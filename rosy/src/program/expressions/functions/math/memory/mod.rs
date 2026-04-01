//! # Memory Estimation Functions (COSY Compatibility)
//!
//! These functions exist for COSY INFINITY compatibility. In Rosy, memory
//! management is handled automatically, so these return compatibility values
//! (e.g., LRE returns 1, LVE returns n, LDA returns the binomial coefficient).
//!
//! | Function | Description |
//! |----------|-------------|
//! | `LST(x)` | String memory estimate |
//! | `LCM(x)` | Complex memory estimate |
//! | `LCD(x)` | Complex DA memory estimate |
//! | `LRE(x)` | Real memory estimate |
//! | `LLO(x)` | Logical memory estimate |
//! | `LVE(x)` | Vector memory estimate |
//! | `LDA(x)` | DA memory estimate |

pub mod lst;
pub mod lcm;
pub mod lcd;
pub mod lre;
pub mod llo;
pub mod lve;
pub mod lda;
