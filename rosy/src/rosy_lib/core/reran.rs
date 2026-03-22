use rand::Rng;

/// Returns a pseudo-random f64 in [-1, 1], matching COSY's RERAN behavior.
pub fn rosy_reran() -> f64 {
    rand::rng().random_range(-1.0..=1.0)
}
