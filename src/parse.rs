//! This module is responsible for parsing different files
//! and returning usable data
/// Defines wrappers for handling todo items
pub mod todo {
    use std::fmt::Debug;
    use std::ops::{Index, IndexMut};

    use crate::{ordered_list::OrderedList, Score};

    use super::ListItem;

    /// A single Todo-item
    #[derive(Debug, Default, Clone)]
    pub struct Item {
        /// The title of this todo item
        pub title: Box<str>,
        /// A description of what this todo item entails
        pub description: Box<str>,
        /// How far one has scrolled in the description
        pub description_scroll: usize,
        // TODO: Add a tag system
    }

    impl ListItem for Item {
        fn title(&self) -> String {
            self.title.to_string()
        }

        fn description(&self) -> String {
            self.description.to_string()
        }
    }

    /// A list of items
    #[derive(Debug, Default, Clone)]
    pub struct Items<T>
    where
        T: Debug + Score + ListItem,
    {
        /// TODO: This shouln't be public
        /// look at how Box does its thing
        pub items: OrderedList<T>,
    }

    impl<T> Items<T>
    where
        T: Debug + Score + ListItem,
    {
        /// Returns the amount of items left
        #[must_use]
        pub fn amount(&self) -> usize {
            self.items.len()
        }

        /// Adds an item
        pub fn add(&mut self, item: T) {
            self.items.insert(item);
        }
        /// Removes an item
        pub fn remove(&mut self, index: usize) -> T {
            self.items.remove(index)
        }
        /// Check if there are any items left
        #[must_use]
        pub fn is_empty(&self) -> bool {
            self.items.is_empty()
        }

        /// Returns a list of each title
        #[must_use]
        pub fn titles(&self) -> Vec<String> {
            self.items.iter().map(super::ListItem::title).collect()
        }
    }

    impl<T> Index<usize> for Items<T>
    where
        T: Debug + Score + ListItem,
    {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.items[index]
        }
    }

    impl<T> IndexMut<usize> for Items<T>
    where
        T: Debug + Score + ListItem,
    {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.items[index]
        }
    }

    impl<T> FromIterator<T> for Items<T>
    where
        T: Debug + Score + ListItem,
    {
        fn from_iter<U>(iter: U) -> Self
        where
            U: IntoIterator<Item = T>,
        {
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

/// Defines a trait which has a peeking value or a `title` and a description
pub trait ListItem {
    /// The title of the item. This is shon in lists and such
    fn title(&self) -> String;
    /// The description of the item. This gets shown when the title is selected
    fn description(&self) -> String;
}
