# bipack_ru

This is Bipack format implementation, minimalistic by purpose.

> work in progress.

## Already implemented:

The following parts are already safe to use

- u8, u16, u32, u64, `smartint` variable-length unsigned
- i8, i16, i32, i64, `smartint` variable-length signed
- strings (utf8, variable length)
- fixed byte arrays
- variable length byte arrays

The sample code (see `src/lib.rs` for more:)
```rust
fn test() {
    let mut data = Vec::<u8>::new();
    data.put_str("Hello, rupack!");
    println!("size ${}\n{}", data.len(), to_dump(&data));
    let mut src = SliceSource::from(&data);
    assert_eq!("Hello, rupack!", src.get_str().unwrap());
}
```

## Tools and macros

- `to_dump` to convert binary slice into human-readable dump
- 'StringBuilder' super minimalistic string builder (footprint). 


At the moment it does not include `serde` module as it is yet unclear how much
it will increase .wasm size. Could be added later.

The autodoc documentation is good enough already, so we do not repeat it here now.

## How to

- just ad this package to your dependencies, it is on crates.io.

# License

For compliance with other modules this work is provided under APACHE 2.0 license a copy of which is included in the file `LICENSE`.