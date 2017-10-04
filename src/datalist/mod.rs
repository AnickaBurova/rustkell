//! Operations on 'list'
/**
 * File: src/datalist/mod.rs
 * Author: Anicka Burova <anicka.burova@gmail.com>
 * Date: 04.10.2017
 * Last Modified Date: 04.10.2017
 * Last Modified By: Anicka Burova <anicka.burova@gmail.com>
 */


pub mod tails;

pub use self::tails::Tails;
use std::marker::Sized;

/// Operations on 'list'
pub trait DataList {

    /// The tails function returns all final segments of the list, longest first.
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

    fn tails(&self) -> Tails<Self>
        where Self: Sized;
}


impl<T> DataList for Vec<T> {
    fn tails(&self) -> Tails<Self> {
        Tails::create(&self)
    }
}

use std::slice::Iter;

impl<'a, T> DataList for Iter<'a, T> {
    fn tails(&self) -> Tails<Self> {
        Tails::create(&self)
    }
}

impl<'a, T> DataList for &'a[T] {
    fn tails(&self) -> Tails<Self> {
        Tails::create(&self)
    }
}

#[test]
fn slice_test_tails() {
    let v = [1,2,3,4];
    let b = vec![
        vec![1,2,3,4],
        vec![2,3,4],
        vec![3,4],
        vec![4],
        vec![],
    ];
    for (a,b) in (&v[..]).tails().into_iter().zip(b) {
        assert_eq!(a,&b[..]);
    }
}


#[test]
fn iter_test_tails() {
    let v = [1,2,3,4];
    let b = vec![
        vec![1,2,3,4],
        vec![2,3,4],
        vec![3,4],
        vec![4],
        vec![],
    ];
    for (a,b) in v.iter().tails().into_iter().zip(b) {
        let a = a.into_iter().map(|x| *x).collect::<Vec<_>>();
        assert_eq!(a,b);
    }
}


#[test]
fn vector_test_tails() {
    let v = vec![1,2,3,4];
    let b = vec![
        vec![1,2,3,4],
        vec![2,3,4],
        vec![3,4],
        vec![4],
        vec![],
    ];
    for (a,b) in v.tails().into_iter().zip(b) {
        assert_eq!(a,&b[..]);
    }
}

#[test]
fn slice_test_tails_empty() {
    let v: [i32;0] = [];
    let b: Vec<Vec<i32>> = vec![
        vec![],
    ];
    for (a,b) in (&v[..]).tails().into_iter().zip(b) {
        assert_eq!(a,&b[..]);
    }
}

#[test]
fn vector_test_tails_empty() {
    let v: Vec<i32> = vec![];
    let b: Vec<Vec<i32>> = vec![
        vec![],
    ];
    for (a,b) in v.tails().into_iter().zip(b) {
        assert_eq!(a,&b[..]);
    }
}

#[test]
fn iter_test_tails_empty() {
    let v: Vec<i32> = vec![];
    let b: Vec<Vec<i32>> = vec![
        vec![],
    ];
    for (a,b) in v.iter().tails().into_iter().zip(b) {
        let a = a.into_iter().map(|x| *x).collect::<Vec<_>>();
        assert_eq!(a,b);
    }
}
