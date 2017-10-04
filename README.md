# Rustkell is haskell like functions in rust. [![Build Status](https://travis-ci.org/AnickaBurova/rustkell.svg?branch=master)](https://travis-ci.org/AnickaBurova/rustkell) [![Crates.io](https://img.shields.io/crates/v/rustkell.svg)](https://crates.io/crates/rustkell)
More functions will come as I will need them.

## [Documentation](https://docs.rs/rustkell/0.2.1/rustkell/)


* tails: The tails function returns all final segments of the list, longest first.


## Usage
Cargo.toml
```toml
[dependencies]
rustkell = "0.2"
```

main.rs
```rust
extern crate rustkell;

use rustkell::DataList;
use std::iter::Iterator;

fn main() {
    let v = vec![1,2,3,4];
    for t in v.tails() {
        println!("{:?}", t);
    }
}
```
Output:
```shell
> [1, 2, 3, 4]  
> [2, 3, 4]  
> [3, 4]  
> [4]  
> []  
```

## License

Licensed under the MIT license, see [LICENSE](LICENSE)
