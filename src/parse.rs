//! This module is responsible for parsing different files
//! and returning usable data
/// Defines wrappers for handling todo items
pub mod todo {
    use std::ops::{Index, IndexMut};

    use crate::{ordered_list::OrderedList, Score};

    /// A single Todo-item
    #[derive(Debug)]
    pub struct Item {
        /// The title of this todo item
        pub title: Box<str>,
        /// A description of what this todo item entails
        pub description: Box<str>,
        /// How far one has scrolled in the description
        pub description_scroll: usize,
        // TODO: Add a tag system
    }

    /// A list of items
    #[derive(Debug, Default)]
    pub struct Items {
        items: OrderedList<Item>,
    }

    impl Items {
        /// Returns the amount of items left
        #[must_use]
        pub fn amount(&self) -> usize {
            self.items.data.len()
        }

        /// Adds an item
        pub fn add(&mut self, item: Item) {
            self.items.insert(item);
        }
        /// Removes an item
        pub fn remove(&mut self, index: usize) -> Item {
            self.items.data.remove(index)
        }
        /// Check if there are any items left
        #[must_use]
        pub fn is_empty(&self) -> bool {
            self.items.data.is_empty()
        }

        /// Returns a list of each title
        #[must_use]
        pub fn titles(&self) -> Vec<String> {
            self.items
                .data
                .iter()
                .map(|x| x.title.to_string())
                .collect()
        }
    }

    impl Index<usize> for Items {
        type Output = Item;

        fn index(&self, index: usize) -> &Self::Output {
            &self.items.data[index]
        }
    }

    impl IndexMut<usize> for Items {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.items.data[index]
        }
    }

    impl FromIterator<Item> for Items {
        fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
            Self {
                items: iter.into_iter().collect(),
            }
        }
    }

    impl From<(Box<str>, (Box<str>, usize))> for Item {
        fn from((title, (description, description_scroll)): (Box<str>, (Box<str>, usize))) -> Self {
            Self {
                title,
                description,
                description_scroll,
            }
        }
    }

    impl Score for Item {
        fn score(&self, query: &str) -> Option<i64> {
            if (self.title.to_string() + &self.description_scroll.to_string()).contains(query) {
                Some(
                    (self.title.len() + self.description.len())
                        .try_into()
                        .unwrap(),
                )
            } else {
                None
            }
        }
    }
}
