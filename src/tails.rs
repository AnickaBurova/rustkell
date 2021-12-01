//! The tails function returns all final segments of the list, longest first.
//! # Example
//! ```
//! use rustkell::DataList;
//! let v = vec![1,2,3,4];
//! for t in v.tails() {
//!     println!("{:?}", t);
//! }
//! ```
//! > [1, 2, 3, 4]
//! > [2, 3, 4]
//! > [3, 4]
//! > [4]
//! > []


pub struct Tails<T, I: Iterator<Item = T>> {
    list: I,
    last: bool,
}

impl<T, I: Iterator<Item=T>> Tails<T, I> {
    pub fn create(list: I) -> Self {
        Self {
            list, last: false,
        }
    }
}

pub enum TailsList<T, I: Iterator<Item = T>> {
    Empty,
    Iter(Option<T>, I),
}

impl<T, I: Iterator<Item=T>> Iterator for TailsList<T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            TailsList::Empty => None,
            TailsList::Iter(first, next) => {
                if let Some(first) = first.take() {
                    Some(first)
                } else {
                    next.next()
                }
            }
        }
    }
}

impl<T, I: Iterator<Item=T> + Clone> Iterator for Tails<T, I> {
    type Item = TailsList<T, I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last {
            None
        } else {
            if let Some(elem) = self.list.next() {
                let next = self.list.clone();
                Some(TailsList::Iter(Some(elem), next))
            } else {
                self.last = true;
                Some(Self::Item::Empty)
            }
        }
    }
}

