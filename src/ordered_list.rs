//! A data structure that keeps an ordered list based off the Score trait
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::Score;

/// The main struct of this module
pub struct OrderedList<T>
where
    T: Debug + Score,
{
    data: Vec<T>,
}

impl<T> Clone for OrderedList<T>
where
    T: Clone + Score + Debug,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T> Debug for OrderedList<T>
where
    T: Debug + Score,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OrderedList")
            .field("data", &self.data)
            .finish()
    }
}

impl<T> Default for OrderedList<T>
where
    T: Debug + Score,
{
    fn default() -> Self {
        Self {
            data: Vec::default(),
        }
    }
}

impl<T> FromIterator<T> for OrderedList<T>
where
    T: Debug + Score,
{
    fn from_iter<P: IntoIterator<Item = T>>(iter: P) -> Self {
        let mut data = iter
            .into_iter()
            .filter(|x| x.score("").is_some())
            .collect::<Vec<_>>();
        data.sort_by_cached_key(|x| x.score("").unwrap());
        Self { data }
    }
}

impl<T> OrderedList<T>
where
    T: Debug + Score,
{
    /// Adds a value to the orderedlist keeping it ordered
    pub fn insert(&mut self, data: T) -> usize {
        let pos = self
            .data
            .iter()
            .position(|x| x.score("") < data.score(""))
            .unwrap_or(self.data.len());
        self.data.insert(pos, data);
        pos
    }
}

impl<T> Deref for OrderedList<T>
where
    T: Debug + Score,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for OrderedList<T>
where
    T: Debug + Score,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
