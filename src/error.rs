//! Error handling

/// A helper function to format an error with its source chain.
///
/// This function works with both `&Error` and `Box<dyn Error>`. When passing a boxed error,
/// make sure to dereference it using `&*e`.
pub fn chain(e: &(dyn std::error::Error + 'static)) -> String {
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
