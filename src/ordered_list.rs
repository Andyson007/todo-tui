//! A Linked list implementation

use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
/// An Ordered list
pub struct OrderedList<T> {
    data: Option<Box<Node<T>>>,
}

impl<T> Default for OrderedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> OrderedList<T> {
    /// A new Ordered List
    #[must_use]
    pub const fn new() -> Self {
        Self { data: None }
    }
    /// Creates a new Ordered list from an impl ``IntoIterator``
    /// ```
    /// # use todo::ordered_list::OrderedList;
    /// let _ = OrderedList::from([0, 1, 2, 3]);
    /// ```
    pub fn from(data: impl IntoIterator<Item = T>) -> Self {
        let mut iter = data.into_iter();
        let Some(first) = iter.next() else {
            return Self { data: None };
        };
        let mut first_node = Node {
            data: first,
            next: None,
        };
        let mut curr = &mut first_node;

        for item in iter {
            curr.next = Some(Box::new(Node {
                data: item,
                next: None,
            }));
            curr = unsafe { curr.next.as_mut().unwrap_unchecked() };
        }

        Self {
            data: Some(Box::new(first_node)),
        }
    }

    /// Retruns an iterator
    #[must_use]
    pub const fn iter(&self) -> NodeIterator<T>
    where
        T: Clone,
    {
        NodeIterator {
            next: self.data.as_ref(),
        }
    }
}

impl<'a, T> IntoIterator for &'a OrderedList<T>
where
    T: Clone,
{
    type Item = T;

    type IntoIter = NodeIterator<'a, T> where T: Clone;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Index<usize> for OrderedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut curr = self.data.as_ref().expect("Out of bounds access");
        for _ in 0..index {
            curr = curr.next.as_ref().expect("Out of bounds access");
        }
        &curr.data
    }
}

impl<T> IndexMut<usize> for OrderedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut curr = self.data.as_mut().expect("Out of bounds access");
        for _ in 0..index {
            curr = &mut *curr.next.as_mut().expect("Out of bounds access");
        }
        &mut curr.data
    }
}

/// An iterator that can iterate over nodes
#[derive(Debug)]
pub struct NodeIterator<'a, T>
where
    T: Clone,
{
    #[allow(clippy::borrowed_box)]
    next: Option<&'a Box<Node<T>>>,
}

impl<'a, T> Iterator for NodeIterator<'a, T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = &(self.next?);
        let ret = &next.data;
        self.next = next.next.as_ref();
        Some(ret.clone())
    }
}

impl<T> PartialEq for OrderedList<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        let mut a = self.data.as_ref();
        let mut b = other.data.as_ref();
        while a.is_some() && b.is_some() {
            if unsafe { a.unwrap_unchecked() }.data != unsafe { b.unwrap_unchecked() }.data {
                return false;
            }
            a = unsafe { a.unwrap_unchecked() }.next.as_ref();
            b = unsafe { b.unwrap_unchecked() }.next.as_ref();
        }
        true
    }
}

mod test {
    #[allow(unused_imports)]
    use super::OrderedList;
    #[test]
    fn access_once() {
        let list = OrderedList::from([0, 1, 2, 3, 4, 5]);
        println!("{list:?}");
        assert_eq!(list[1], 1);
        // assert_eq!(list[1], 2);
    }

    #[test]
    fn access_multiple() {
        let list = OrderedList::from([0, 1, 2, 3, 4, 5]);
        println!("{list:?}");
        assert_eq!(list[1], 1);
        assert_eq!(list[1], 1);
        assert_eq!(list[4], 4);
    }

    #[test]
    #[should_panic = "Out of bounds access"]
    fn out_of_bounds() {
        let list = OrderedList::from([0, 1, 2, 3, 4, 5]);
        #[allow(clippy::no_effect)]
        list[6];
    }

    #[test]
    fn modify() {
        let mut list = OrderedList::from([0, 1, 2, 3, 4, 5]);
        list[3] = 4;
        assert_eq!(list[3], 4);
    }

    #[test]
    fn eq() {
        let mut a = OrderedList::from([0, 1, 2, 3, 4, 5]);
        let b = OrderedList::from([0, 1, 2, 3, 4, 5]);
        assert_eq!(a, b);
        a[3] = 4;
        assert_ne!(a, b);
    }
}
