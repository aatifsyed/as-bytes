<div align="center">

[![crates-io](https://img.shields.io/crates/v/as-bytes.svg)](https://crates.io/crates/as-bytes)
[![docs-rs](https://docs.rs/as-bytes/badge.svg)](https://docs.rs/as-bytes)
[![github](https://img.shields.io/static/v1?label=&message=github&color=grey&logo=github)](https://github.com/aatifsyed/as-bytes)

</div>

# `as-bytes`

A tiny crate, which provides slices to the memory which backs an instance of a struct.

```rust
use as_bytes::AsBytes;
let i = u32::MAX;
let bytes = unsafe { i.as_bytes() };
assert_eq!(bytes, [255, 255, 255, 255]);
```
You can use this to peek at structure layout.
One usecase is for testing sending structs sent the wire.
The below examples demonstrate two different packing attributes on the same structure.
```rust

let packed = ReprPacked {
    a: usize::MAX,
    b: 0u8,
    c: usize::MAX,
};
let bytes = unsafe { packed.as_bytes() };
assert_eq!(
    bytes,
    [255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255]
);
```

```rust

let packed = ReprC {
    a: usize::MAX,
    b: 0u8,
    c: usize::MAX,
};
let bytes = unsafe { packed.as_bytes() };
assert_eq!(
    bytes,
    [
        255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255
    ]
);
```
