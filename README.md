# BitArr

[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]

A fast and efficient Rust implementation of a BitSet, supporting multiple backing stores.

## Usage

To use BitArr in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
bitarr = "0"
```

## Example

```rust
use bitarr::BitSet;

let mut bs = BitSet::from(0u16);
bs.set(3);
bs.set(7);

assert_eq!(bs.get(3), Some(true));
assert_eq!(bs.get(7), Some(true));
assert_eq!(bs.get(2), Some(false));

```

## Documentation

API documentation can be found on [docs.rs][docs-url].

## License

BitArr is distributed under the terms of the MIT license.

See LICENSE for details.

[crates-badge]: https://img.shields.io/crates/v/bitarr.svg
[crates-url]: https://crates.io/crates/bitarr
[docs-badge]: https://docs.rs/bitarr/badge.svg
[docs-url]: https://docs.rs/bitarr/
