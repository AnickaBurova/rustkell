# Rustkell is haskell like functions in rust.

More functions will come as I will need them.

## [Documentation](https://docs.rs/rustkell/0.1.0/rustkell/)


* tails: The tails function returns all final segments of the list, longest first.


## Usage
Cargo.toml
```toml
[dependencies]
rustkell = "0.1"
```

main.rs
```rust
extern crate rustkell;

use rustkell::DataList;
use std::iter::Iterator;

fn main() {
    println!("Hello, world!");
    let v = vec![1,2,3,4];
    for t in v.tails() {
        println!("{:?}", t.into_iter().collect::<Vec<_>>());
    }
}
```

## License

Licensed under the MIT license, see [LICENSE](LICENSE)
