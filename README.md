[![pipeline](https://github.com/d-e-s-o/serde_variant/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/d-e-s-o/serde_variant/actions/workflows/test.yml)
[![crates.io](https://img.shields.io/crates/v/serde_variant.svg)](https://crates.io/crates/serde_variant)
[![Docs](https://docs.rs/serde_variant/badge.svg)](https://docs.rs/serde_variant)
[![rustc](https://img.shields.io/badge/rustc-1.57+-blue.svg)](https://blog.rust-lang.org/2021/12/02/Rust-1.57.0.html)

serde_variant
=============

- [Documentation][docs-rs]
- [Changelog](CHANGELOG.md)

So you have just carefully defined your `enum` to be serialized and
deserialized using [`serde`][serde] as you intended and now you need an
additional `FromStr` or `Display` implementation that uses the same
names for `enum` variants as `serde` uses? You are reluctant to
duplicate all those definitions in two places?

**serde_variant** is a crate that allows you to retrieve back the
identifier of any `enum` variant passed to it.


Usage
-----

The crate provides a single function, `to_variant_name`, that retrieves
the name of a passed in `enum` variant. For example:
```rust
use serde_variant::to_variant_name;

#[derive(Serialize)]
enum Foo {
  Var1,
  #[serde(rename = "VAR2")]
  Var2,
}

assert_eq!(to_variant_name(&Foo::Var1).unwrap(), "Var1");
assert_eq!(to_variant_name(&Foo::Var2).unwrap(), "VAR2");
```

[docs-rs]: https://docs.rs/crate/serde_variant
[serde]: https://crates.io/crates/serde
