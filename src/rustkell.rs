//! Rustkell trait which implements some helper functions over "lists"



use pairs::Pairs;
use tails::Tails;

pub trait Rustkell<T>: Iterator<Item = T> + Sized {
    /// Returns tails of the iterator, starting with the full iterator and next is the rest but missing the first from the previous
    /// ```
    ///     use rustkell::Rustkell;
    ///     let v = vec![1,2,3,4,5];
    ///     let t = v.iter().tails();
    ///     for (i, t) in t.enumerate() {
    ///        println!("{}: {:?}", i, t.collect::<Vec<_>>());
    ///     }
    /// ```
    /// > 0: [1, 2, 3, 4, 5]
    /// > 1: [2, 3, 4, 5]
    /// > 2: [3, 4, 5]
    /// > 3: [4, 5]
    /// > 4: [5]
    /// > 5: []
    fn tails(self) -> Tails<T, Self> {
        Tails::create(self)
    }

    /// Returns an iterator over tuples of pairs of elements, pairing them as current and the next.
    /// Returns an empty iterator if the list has less then 2 elements.
    /// ```
    ///     use rustkell::Rustkell;
    ///     let v = vec![1,2,3,4];
    ///     for e in v.iter().pairs() {
    ///         println!("{:?}", e);
    ///     }
    /// ```
    /// > (1,2)
    /// > (2,3)
    /// > (3,4)
    fn pairs(self) -> Pairs<T, Self> {
        Pairs::create(self)
    }
}

impl<T, I: Iterator<Item = T> + Sized> Rustkell<T> for I {

}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, VecDeque};
    use super::*;
    #[test]
    fn tails() {
        let v = vec![1,2,3,4,5];
        let mut t = v.into_iter().tails();
        let p = t.next().unwrap().collect::<Vec<_>>();
        assert_eq!(p, vec![1,2,3,4,5]);
        let p = t.next().unwrap().collect::<Vec<_>>();
        assert_eq!(p, vec![2,3,4,5]);
        let p = t.next().unwrap().collect::<Vec<_>>();
        assert_eq!(p, vec![3,4,5]);
        let p = t.next().unwrap().collect::<Vec<_>>();
        assert_eq!(p, vec![4,5]);
        let p = t.next().unwrap().collect::<Vec<_>>();
        assert_eq!(p, vec![5]);
        let p = t.next().unwrap().collect::<Vec<_>>();
        assert!(p.is_empty());
        let h = HashMap::<String, &str>::new();
        // h.insert("one", 1);
        // h.insert("two", 2);
        // h.insert("three", 3);
        let _ = h.keys().tails();
        let _ = h.values().tails();
        let _ = h.iter().tails();
        let _ = h.into_iter().enumerate().tails();

        let dq = VecDeque::<i32>::new();
        let _ = dq.iter().tails();
        // for (i, t) in t.enumerate() {
        //     println!("{}: {:?}", i, t.collect::<Vec<_>>());
        // }
    }

    #[test]
    fn pairs() {
        let v = vec![1,2,3,4,5];
        let p = v.iter().pairs().collect::<Vec<_>>();
        assert_eq!(p.len(), 4);
        assert_eq!(p[0], (&1,&2));
        assert_eq!(p[1], (&2,&3));
        assert_eq!(p[2], (&3,&4));
        assert_eq!(p[3], (&4,&5));

        let v = vec![1,2,3,4,5];
        let p = v.into_iter().pairs().collect::<Vec<_>>();
        assert_eq!(p.len(), 4);
        assert_eq!(p[0], (1,2));
        assert_eq!(p[1], (2,3));
        assert_eq!(p[2], (3,4));
        assert_eq!(p[3], (4,5));

        let v = vec![1];
        let p = v.iter().pairs().collect::<Vec<_>>();
        assert!(p.is_empty());

        let v = Vec::<String>::new();
        let p = v.iter().pairs().collect::<Vec<_>>();
        assert!(p.is_empty());

        let v = vec!["hello".to_owned(), "world".to_owned()];
        let p = v.into_iter().pairs().collect::<Vec<_>>();
        assert_eq!(p[0], ("hello".to_owned(), "world".to_owned()));
    }
}