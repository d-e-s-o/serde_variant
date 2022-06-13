// Copyright (C) 2020-2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use serde::de;
use serde::ser;
use serde::ser::Error as _;
use serde::Deserialize;
use serde::Serialize;

/// An error emitted when attempting to perform an unsupported
/// operation.
#[derive(Clone, Debug, PartialEq)]
pub struct UnsupportedType {
    /// The method that is not supported.
    method: String,
}

impl Display for UnsupportedType {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        write!(fmt, "{} is not supported", self.method)
    }
}

impl Error for UnsupportedType {}

impl ser::Error for UnsupportedType {
    fn custom<T>(method: T) -> Self
    where
        T: Display,
    {
        Self {
            method: method.to_string(),
        }
    }
}

/// Convert an enum variant into its name.
///
/// Note that only enum variants may be converted here and all other
/// types will result in an `UnsupportedType` error.
pub fn to_str<T>(value: &T) -> Result<&'static str, UnsupportedType>
where
    T: Serialize,
{
    let mut serializer = Serializer {};
    value.serialize(&mut serializer)
}

/// A serializer for tuple enum variants.
struct TupleVariantSerializer(&'static str);

impl ser::SerializeTupleVariant for TupleVariantSerializer {
    type Ok = &'static str;
    type Error = UnsupportedType;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.0)
    }
}

/// A serializer for struct enum variants.
struct StructVariantSerializer(&'static str);

impl ser::SerializeStructVariant for StructVariantSerializer {
    type Ok = &'static str;
    type Error = UnsupportedType;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.0)
    }
}

/// A serde serializer that converts an enum variant into the variant's
/// name.
struct Serializer {}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = &'static str;
    type Error = UnsupportedType;

    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = TupleVariantSerializer;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = StructVariantSerializer;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_bool"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_i8"))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_i16"))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_i32"))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_i64"))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_u8"))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_u16"))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_u32"))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_u64"))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_f32"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_f64"))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_char"))
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_str"))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_bytes"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_none"))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::custom("serialize_unit"))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(name)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Self::Error::custom("serialize_seq"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Self::Error::custom("serialize_tuple"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Self::Error::custom("serialize_tuple_struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(TupleVariantSerializer(variant))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Self::Error::custom("serialize_map"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Self::Error::custom("serialize_struct"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(StructVariantSerializer(variant))
    }

    fn collect_str<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized,
    {
        Err(Self::Error::custom("collect_str"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Ok(name)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Ok(variant)
    }
}

pub fn from_str<'a, E>(value: &'static str) -> Result<E, DeserializationError>
where
    E: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(value);
    let variant = E::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(variant)
    } else {
        Err(DeserializationError::InvalidVariantName {
            name: value.to_string(),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DeserializationError {
    UnsupportedType(UnsupportedType),
    InvalidVariantName { name: String },
}

impl Display for DeserializationError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::InvalidVariantName { name } => {
                write!(fmt, "{} is not a valid variant name", name)
            }
            Self::UnsupportedType(ut) => write!(fmt, "{}", ut),
        }
    }
}

impl Error for DeserializationError {}

impl de::Error for DeserializationError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        if msg.to_string().starts_with("serialize_") {
            return Self::UnsupportedType(UnsupportedType {
                method: msg.to_string(),
            });
        }

        Self::InvalidVariantName {
            name: msg.to_string(),
        }
    }
}

pub struct Deserializer<'de> {
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = DeserializationError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn is_human_readable(&self) -> bool {
        true
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        EnumVariantVisitor
    }
}

struct EnumVariantVisitor<E>(PhantomData<E>);

impl<'de, E> de::Visitor<'de> for EnumVariantVisitor<E> {
    type Value = E;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid enum variant name")
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        //let variant = data.variant()?;
        unimplemented!()
    }
}

struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Enum { de }
    }
}

impl<'de, 'a> de::EnumAccess<'de> for Enum<'a, 'de> {
    type Error = DeserializationError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        // The `deserialize_enum` method parsed a `{` character so we are
        // currently inside of a map. The seed will be deserializing itself from
        // the key of the map.
        let val = seed.deserialize(&mut *self.de)?;
        // Parse the colon separating map key from value.
        if self.de.next_char()? == ':' {
            Ok((val, self))
        }
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
    }
}
