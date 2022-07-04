//! Error handling

/// A helper to format error with its source chain
pub fn chain(e: &impl std::error::Error) -> String {
    use std::fmt::Write as _;

    let mut s = e.to_string();
    let mut current = e.source();
    if current.is_some() {
        s.push_str("\nCaused by:");
    }
    while let Some(cause) = current {
        write!(s, "\n\t{}", cause).ok();
        current = cause.source();
    }
    s
}
