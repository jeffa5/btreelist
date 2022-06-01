#![forbid(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_docs, // TODO: add documentation!
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

//! See [`BTreeList`].

mod btreelist;
mod iter;

pub use crate::btreelist::BTreeList;
pub use crate::iter::Iter;
