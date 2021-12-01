//! An iterator over a list, which returns pairs of the current and the next element.
//! If there is only one element in the list, the result in empty
//!

/// An iterator over a list, which returns elements paired in a sequence
pub struct Pairs<T, I: Iterator<Item = T>> {
    list: I,
    prev: Option<T>,
}


impl<T: Copy, I: Iterator<Item=T>> Iterator for Pairs<T, I> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prev) = self.prev.take() {
            if let Some(current) = self.list.next() {
                self.prev = Some(current);
                Some((prev, current))
            } else {
                None
            }
        } else {
            if let Some(prev) = self.list.next() {
                self.prev = Some(prev);
                self.next()
            } else {
                None
            }
        }
    }
}


impl<T, I: Iterator<Item=T>> Pairs<T, I> {
    pub fn create(list: I) -> Self {
        Self {
            list,
            prev: None,
        }
    }
}


