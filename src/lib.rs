// Copyright (C) 2020-2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

mod de;
mod ser;

use serde::Deserialize;
use serde::Serialize;

pub use de::Deserializer;
pub use ser::Serializer;

/// Convert an enum variant into its name.
///
/// Note that only enum variants and unit structs may be
/// converted here and all other types will result in an
/// `UnsupportedType` error.
pub fn to_str<T>(value: &T) -> Result<&'static str, ser::UnsupportedType>
where
    T: Serialize,
{
    let mut serializer = ser::Serializer {};
    value.serialize(&mut serializer)
}

/// Convert a str into an enum
///
/// Note that only unit enum variants and unit structs may be
/// constructed from a string
pub fn from_str<'a, E>(value: &'static str) -> Result<E, de::DeserializationError>
where
    E: Deserialize<'a>,
{
    let mut deserializer = de::Deserializer::from_str(value);
    let variant = E::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(variant)
    } else {
        Err(de::DeserializationError::InvalidVariantName {
            name: value.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod ser {
        use super::*;

        #[test]
        fn unit_variant_names() {
            #[derive(Serialize)]
            enum Foo {
                Var1,
                #[serde(rename = "VAR2")]
                Var2,
            }

            assert_eq!(to_str(&Foo::Var1).unwrap(), "Var1");
            assert_eq!(to_str(&Foo::Var2).unwrap(), "VAR2");
        }

        #[test]
        fn newtype_variant_names() {
            #[derive(Serialize)]
            enum Foo {
                Var1(()),
                #[serde(rename = "VAR2")]
                Var2(u32),
            }

            assert_eq!(to_str(&Foo::Var1(())).unwrap(), "Var1");
            assert_eq!(to_str(&Foo::Var2(42)).unwrap(), "VAR2");
        }

        #[test]
        fn tuple_variant_names() {
            #[derive(Serialize)]
            enum Foo {
                BAz((), u64),
                #[serde(rename = "VAR")]
                Var((), (), ()),
            }

            assert_eq!(to_str(&Foo::BAz((), 1337)).unwrap(), "BAz");
            assert_eq!(to_str(&Foo::Var((), (), ())).unwrap(), "VAR");
        }

        #[test]
        fn unit_struct() {
            #[derive(Serialize)]
            struct Bar;

            assert_eq!(to_str(&Bar).unwrap(), "Bar");
        }

        #[test]
        fn newtype_struct() {
            #[derive(Serialize)]
            struct Bar(u64);

            assert_eq!(to_str(&Bar(42)).unwrap(), "Bar");
        }
    }

    mod de {
        use super::*;

        #[test]
        fn unit_variant_names() {
            #[derive(Debug, PartialEq, Deserialize)]
            enum Foo {
                Var1,
                #[serde(rename = "VAR2")]
                Var2,
            }

            assert_eq!(from_str::<Foo>("Var1").unwrap(), Foo::Var1);
            assert_eq!(from_str::<Foo>("VAR2").unwrap(), Foo::Var2);
        }

        #[test]
        fn tuple_variant_names() {
            #[derive(Debug, PartialEq, Deserialize)]
            enum Foo {
                BAz(u8),
                #[serde(rename = "VAR")]
                Var((), (), u8),
            }

            assert!(from_str::<Foo>("BAz").is_err());
            assert!(from_str::<Foo>("VAR").is_err());
        }

        #[test]
        fn unit_struct() {
            #[derive(Debug, Deserialize, PartialEq)]
            struct Bar;

            assert_eq!(from_str::<Bar>("Bar").unwrap(), Bar);
        }

        #[test]
        fn unit_struct_renamed() {
            #[derive(Debug, Deserialize, PartialEq)]
            #[serde(rename = "BAR")]
            struct Bar;

            assert_eq!(from_str::<Bar>("BAR").unwrap(), Bar);
            assert!(from_str::<Bar>("bAR").is_err());
            assert!(from_str::<Bar>("bar").is_err());
        }
    }
}
