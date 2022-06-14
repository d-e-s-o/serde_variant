[![pipeline](https://gitlab.com/d-e-s-o/serde_variant/badges/master/pipeline.svg)](https://gitlab.com/d-e-s-o/serde_variant/commits/master)
[![crates.io](https://img.shields.io/crates/v/serde_variant.svg)](https://crates.io/crates/serde_variant)
[![Docs](https://docs.rs/serde_variant/badge.svg)](https://docs.rs/serde_variant)
[![rustc](https://img.shields.io/badge/rustc-1.31+-blue.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)

# serde_variant

- [Documentation][docs-rs]
- [Changelog](CHANGELOG.md)

So you have just carefully defined your `enum` to be serialized and
deserialized using [`serde`][serde] as you intended and now you need an
additional `FromStr` or `Display` implementation that uses the same
names for `enum` variants as `serde` uses? You are reluctant to
duplicate all those definitions in two places?

**serde_variant** is a crate that allows you to retrieve back the
identifier of any `enum` variant passed to it.

## Usage

The crate provides the two functions `to_string` and `from_str` which can be used
to serialize types into their names and even deserialize them again if they
didnt held any data:

```rust
use serde::{Serialize, Deserialize};
use serde_variant::{to_string, from_str};

#[derive(Serialize, Deserialize)]
enum Foo {
  UnitVariant,
  #[serde(rename = "RENAMED")]
  Renamed,
  HoldsData(u8),
  HoldsDataAsTuple(u8, u8),
  HoldsDataAsStruct { field: u8 }
}

// Safe to serialize, held data gets discarded
assert_eq!(to_string(&Foo::UnitVariant).unwrap(), "UnitVariant");
assert_eq!(to_string(&Foo::Renamed).unwrap(), "RENAMED");
assert_eq!(to_string(&Foo::HoldsData(0)).unwrap(), "HoldsData");
assert_eq!(to_string(&Foo::HoldsDataAsTuple(0, 0)).unwrap(), "HoldsDataAsTuple");
assert_eq!(to_string(&Foo::HoldsDataAsStruct { field: 0 }).unwrap(), "HoldsDataAsStruct");

// Safe to deserialize since no data is held
assert_eq!(from_str::<Foo>("UnitVariant").unwrap(), Foo::UnitVariant);
assert_eq!(from_str::<Foo>("RENAMED").unwrap(), Foo::Renamed);
// Cant be deserialized since their data was lost during serialization
assert!(from_str::<Foo>("HoldsData").is_err());
assert!(from_str::<Foo>("HoldsDataAsTuple").is_err());
assert!(from_str::<Foo>("HoldsDataAsStruct").is_err());
```

[docs-rs]: https://docs.rs/crate/serde_variant
[serde]: https://crates.io/crates/serde
