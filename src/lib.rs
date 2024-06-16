//! Imports all modules
#![deny(
    missing_docs,
    missing_abi,
    missing_fragment_specifier,
    missing_debug_implementations
)]
#![warn(clippy::pedantic, clippy::nursery)]

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
    T: Score,
{
    let mut indexed = iter.into_iter().enumerate().collect::<Vec<_>>();
    indexed.sort_by_key(|x| x.1.score(query));
    indexed
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
            Some(1)
        } else {
            None
        }
    }
}
