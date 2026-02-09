//! Regression test for #98291: tool attributes on `macro_export` macros should compile.
//! See also issue #150518 for a similar case.
//!
//! Tool attributes like `#[rustfmt::skip]`, `#[clippy::format_args]`, and
//! `#[rust_analyzer::macro_style]` are inert -- they do not transform the item.
//! They should not trigger the `macro_expanded_macro_exports_accessed_by_absolute_paths` lint.

//@ check-pass

// Case 1: #[rustfmt::skip] before #[macro_export], used via re-export.
#[rustfmt::skip]
#[macro_export]
macro_rules! _a {
    () => { "Hello world" };
}

pub use _a as a;

// Case 2: #[rust_analyzer::macro_style] after #[macro_export], used via $crate absolute path.
#[macro_export]
#[rust_analyzer::macro_style(braces)]
macro_rules! match_ast {
    (match $node:ident {}) => { $crate::match_ast!(match ($node) {}) };
    (match ($node:expr) {}) => {};
}

// Case 3: #[clippy::format_args] on a macro used from a submodule via absolute path.
#[clippy::format_args]
#[macro_export]
macro_rules! my_log {
    ($($t:tt)*) => { format_args!($($t)*) }
}

mod sub {
    pub fn run() {
        let _ = crate::my_log!("Hello, world!");
    }
}

fn main() {
    println!(a!());

    match_ast! {
        match foo {}
    }

    sub::run();
}
