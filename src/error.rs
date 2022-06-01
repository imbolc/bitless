//! Error handling

/// A helper to format error with its source chain
pub fn chain(e: &impl std::error::Error) -> String {
    let mut s = e.to_string();
    let mut current = e.source();
    if current.is_some() {
        s.push_str("\nCaused by:");
    }
    while let Some(cause) = current {
        s.push_str(&format!("\n\t{}", cause));
        current = cause.source();
    }
    s
}
