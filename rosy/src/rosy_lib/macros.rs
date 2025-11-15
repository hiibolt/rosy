/// Macro to conditionally include a file as a string, returning empty string if it doesn't exist.
/// This is useful for documentation that references generated files which may not exist
/// in vendored/embedded contexts.
#[macro_export]
macro_rules! include_str_optional {
    ($path:expr) => {
        if std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path)).exists() {
            include_str!($path)
        } else {
            ""
        }
    };
}
