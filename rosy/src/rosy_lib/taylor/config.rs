//! Global configuration for Taylor series computations.

use std::sync::RwLock;
use anyhow::{Result, Context, bail};

use super::DEFAULT_EPSILON;

/// Configuration for Taylor series computations.
#[derive(Debug, Clone, Copy)]
pub struct TaylorConfig {
    /// Maximum order of polynomials
    pub max_order: u32,
    /// Number of variables
    pub num_vars: usize,
    /// Epsilon threshold for coefficient truncation
    pub epsilon: f64,
}

impl TaylorConfig {
    /// Create a new Taylor configuration.
    ///
    /// # Arguments
    /// * `max_order` - Maximum polynomial order
    /// * `num_vars` - Number of variables (must be <= MAX_VARS)
    /// * `epsilon` - Truncation threshold for small coefficients
    ///
    /// # Returns
    /// A new configuration, or error if num_vars exceeds MAX_VARS
    pub fn new(max_order: u32, num_vars: usize, epsilon: f64) -> Result<Self> {
        if num_vars > super::MAX_VARS {
            bail!(
                "Number of variables ({}) exceeds maximum ({})",
                num_vars,
                super::MAX_VARS
            );
        }
        
        Ok(Self {
            max_order,
            num_vars,
            epsilon,
        })
    }
}

/// Global Taylor configuration (COSY-style global state for easier transpilation)
static TAYLOR_CONFIG: RwLock<Option<TaylorConfig>> = RwLock::new(None);

/// Initialize the Taylor series system.
///
/// Must be called before any DA/CD operations.
///
/// # Arguments
/// * `max_order` - Maximum order of Taylor expansions
/// * `num_vars` - Number of variables
///
/// # Returns
/// Ok(()) on success, or error if already initialized or invalid parameters
///
/// # Example
/// ```
/// use rosy_lib::taylor::init_taylor;
/// 
/// init_taylor(10, 3)?;  // Order 10, 3 variables
/// ```
pub fn init_taylor(max_order: u32, num_vars: usize) -> Result<()> {
    let mut config = TAYLOR_CONFIG.write()
        .map_err(|e| anyhow::anyhow!("Failed to acquire config lock: {}", e))?;
    
    if config.is_some() {
        bail!("Taylor system already initialized. Call cleanup_taylor() first.");
    }
    
    *config = Some(TaylorConfig::new(max_order, num_vars, DEFAULT_EPSILON)?);
    Ok(())
}

/// Get the current Taylor configuration.
///
/// # Returns
/// The current configuration, or error if not initialized
pub fn get_config() -> Result<TaylorConfig> {
    let config = TAYLOR_CONFIG.read()
        .map_err(|e| anyhow::anyhow!("Failed to acquire config lock: {}", e))?;
    
    config.ok_or_else(|| {
        anyhow::anyhow!("Taylor system not initialized. Call init_taylor() first.")
    })
}

/// Set the epsilon value for coefficient truncation.
///
/// # Arguments
/// * `epsilon` - New epsilon value
///
/// # Returns
/// The previous epsilon value, or error if not initialized
pub fn set_epsilon(epsilon: f64) -> Result<f64> {
    let mut config = TAYLOR_CONFIG.write()
        .map_err(|e| anyhow::anyhow!("Failed to acquire config lock: {}", e))?;
    
    let cfg = config.as_mut()
        .ok_or_else(|| anyhow::anyhow!("Taylor system not initialized"))?;
    
    let old_epsilon = cfg.epsilon;
    cfg.epsilon = epsilon;
    Ok(old_epsilon)
}

/// Check if Taylor system is initialized.
pub fn is_initialized() -> bool {
    TAYLOR_CONFIG.read()
        .map(|c| c.is_some())
        .unwrap_or(false)
}

/// Clean up the Taylor system (for re-initialization).
pub fn cleanup_taylor() {
    if let Ok(mut config) = TAYLOR_CONFIG.write() {
        *config = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let cfg = TaylorConfig::new(10, 3, 1e-15).unwrap();
        assert_eq!(cfg.max_order, 10);
        assert_eq!(cfg.num_vars, 3);
        assert_eq!(cfg.epsilon, 1e-15);
    }

    #[test]
    fn test_config_too_many_vars() {
        let result = TaylorConfig::new(10, super::super::MAX_VARS + 1, 1e-15);
        assert!(result.is_err());
    }

    #[test]
    fn test_global_init() {
        cleanup_taylor(); // Clean slate
        
        assert!(!is_initialized());
        
        init_taylor(5, 2).unwrap();
        assert!(is_initialized());
        
        let cfg = get_config().unwrap();
        assert_eq!(cfg.max_order, 5);
        assert_eq!(cfg.num_vars, 2);
        
        cleanup_taylor();
        assert!(!is_initialized());
    }
}
