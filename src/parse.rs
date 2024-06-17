//! This module is responsible for parsing different files
//! and returning usable data
/// Defines wrappers for handling todo items
pub mod todo {
    use std::ops::{Index, IndexMut};

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
        items: Vec<Item>,
    }

    impl Items {
        /// Returns the amount of items left
        #[must_use]
        pub fn amount(&self) -> usize {
            self.items.len()
        }

        /// Adds an item
        pub fn add(&mut self, item: Item) {
            // FIXME: This doesn't maintain a sorted order resulting in distorientation when
            // filtering
            self.items.push(item);
        }
        /// Removes an item
        pub fn remove(&mut self, index: usize) -> Item {
            self.items.remove(index)
        }
        /// Check if there are any items left
        #[must_use]
        pub fn is_empty(&self) -> bool {
            self.items.is_empty()
        }

        // FIXME: This actually forces the internal representation to use RefCell

        /// Returns a list of each title
        #[must_use]
        pub fn titles(&self) -> Vec<String> {
            self.items.iter().map(|x| x.title.to_string()).collect()
        }
    }

    impl Index<usize> for Items {
        type Output = Item;

        fn index(&self, index: usize) -> &Self::Output {
            &self.items[index]
        }
    }

    impl IndexMut<usize> for Items {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.items[index]
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
}
