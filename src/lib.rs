//! A bit less code to carry with almost any program

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

pub mod error;
pub mod journal;

/// Returns the current function path, it could be used in `map_err` or `expect` to avoid typos
#[macro_export]
macro_rules! code_path {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let mut name = type_name_of(f);
        name = &name[..name.len() - 3];
        while name.ends_with("::{{closure}}") {
            name = &name[..name.len() - 13];
        }
        name
    }};
}
