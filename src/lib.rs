#![forbid(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_docs, // TODO: add documentation!
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
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

//! A tree-based list with heap-allocated contents.
//!
//! A BTreeList offers `O(log(n))` indexing, `O(log(n))` insertion (anywhere in the list) and
//! `O(log(n))` removal (also anywhere in the list).
//!
//! See [`BTreeList`] for more details.

mod btreelist;
mod iter;
mod r#macro;
mod owned_iter;

pub use crate::btreelist::BTreeList;
pub use crate::iter::Iter;
pub use crate::owned_iter::OwnedIter;
