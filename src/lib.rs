//! Rustkell is haskell like functions in rust.
/**
 * File: src/lib.rs
 * Author: Anicka Burova <anicka.burova@gmail.com>
 * Date: 04.10.2017
 * Last Modified Date: 04.10.2017
 * Last Modified By: Anicka Burova <anicka.burova@gmail.com>
 */

mod rustkell;
pub use rustkell::Rustkell;

mod datalist;

mod tails;

mod pairs;

pub use datalist::DataList;
pub use datalist::Tails;
