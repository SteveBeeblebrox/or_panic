# or_panic
Extract ok values from Rust results or panic with the contained error.
```rust
use or_panic::OrPanic as _;

println!("{:?}", Path::new("./foo.txt").canonicalize().or_panic());
```