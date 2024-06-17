//! Imports all modules
#![deny(
    missing_docs,
    missing_abi,
    missing_fragment_specifier,
    missing_debug_implementations
)]
#![warn(clippy::pedantic, clippy::nursery)]

use core::fmt::Debug;

pub mod app;
pub mod app_builder;
pub mod errors;
pub mod help;
pub mod ui;

/// Returns an ordered list how alike it is to
/// the search query
#[must_use]
pub fn query<T>(iter: Vec<T>, query: &str) -> Vec<(usize, T)>
where
    T: Score + Clone + Debug,
{
    let mut indexed = iter
        .into_iter()
        .enumerate()
        .filter_map(|x| Some((x.clone(), x.1.score(query)?)))
        .collect::<Vec<_>>();
    indexed.sort_by_key(|x| x.1);
    indexed.into_iter().map(|x| x.0).collect()
}

/// Implements a scoring trait used for ordering the search items
pub trait Score {
    /// The scoring function it should return None if the
    /// search item shouldn't be included in the final list
    fn score(&self, query: &str) -> Option<i64>;
}

impl Score for String {
    fn score(&self, query: &str) -> Option<i64> {
        if self.contains(query) {
            i64::try_from(self.len()).ok()
        } else {
            None
        }
    }
}
