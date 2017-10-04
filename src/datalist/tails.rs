/**
 * File: src/datalist/tails.rs
 * Author: Anicka Burova <anicka.burova@gmail.com>
 * Date: 04.10.2017
 * Last Modified Date: 04.10.2017
 * Last Modified By: Anicka Burova <anicka.burova@gmail.com>
 */

use std::iter::{Iterator, Skip};
use std::slice::Iter;

/// The Tails is returned using tails function returns all final segments of the list, longest first.
/// # Example
/// ```
/// use rustkell::DataList;
/// let v = vec![1,2,3,4];
/// for t in v.tails() {
///     println!("{:?}", t);
/// }
/// ```
/// > [1, 2, 3, 4]  
/// > [2, 3, 4]  
/// > [3, 4]  
/// > [4]  
/// > []  

pub struct Tails<'a, T: 'a> {
    iter: &'a T,
    index: usize,
    last: bool,
}

impl<'a, T: 'a> Tails<'a, T> {
    pub fn create(iter: &'a T) -> Self {
        Tails {
            iter,
            index: 0,
            last: false,
        }
    }
}


impl<'a, T> Iterator for Tails<'a, Vec<T>> {
    type Item = &'a[T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.last {
            None
        } else {
            let res = &self.iter[self.index..];
            self.last = res.len() == 0;
            self.index += 1;
            Some(res)
        }
    }
}

impl<'a, T> Iterator for Tails<'a, Iter<'a, T>> {
    type Item = Skip<Iter<'a,T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last {
            None
        } else {
            // get the result here
            let tmp = self.iter.clone();
            let res = tmp.skip(self.index);
            // check if this is the last one
            // using peekable because I cannot find any other efficient method!
            self.last = res.peekable().peek().is_none();
            // peekable moved our result so need to get it again
            let tmp = self.iter.clone();
            let res = tmp.skip(self.index);
            self.index += 1;
            Some(res)
        }
    }
}
