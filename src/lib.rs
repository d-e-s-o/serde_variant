// Copyright (C) 2020-2024 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use serde::ser::Error as SerdeError;
use serde::ser::Impossible;
use serde::ser::SerializeStructVariant;
use serde::ser::SerializeTupleVariant;
use serde::Serialize;
use serde::Serializer as SerdeSerializer;


/// An error emitted when attempting to perform an unsupported
/// operation.
#[derive(Clone, Debug, PartialEq)]
pub struct UnsupportedType {
  /// The method that is not supported.
  method: String,
}

impl Display for UnsupportedType {
  #[inline]
  fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
    write!(fmt, "{} is not supported", self.method)
  }
}

impl Error for UnsupportedType {}

impl SerdeError for UnsupportedType {
  #[inline]
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
#[inline]
pub fn to_variant_name<T>(value: &T) -> Result<&'static str, UnsupportedType>
where
  T: Serialize,
{
  let mut serializer = Serializer {};
  value.serialize(&mut serializer)
}


/// A serializer for tuple enum variants.
struct TupleVariantSerializer(&'static str);

impl SerializeTupleVariant for TupleVariantSerializer {
  type Ok = &'static str;
  type Error = UnsupportedType;

  #[inline]
  fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
  where
    T: Serialize + ?Sized,
  {
    Ok(())
  }

  #[inline]
  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok(self.0)
  }
}

/// A serializer for struct enum variants.
struct StructVariantSerializer(&'static str);

impl SerializeStructVariant for StructVariantSerializer {
  type Ok = &'static str;
  type Error = UnsupportedType;

  #[inline]
  fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
  where
    T: ?Sized + Serialize,
  {
    Ok(())
  }

  #[inline]
  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok(self.0)
  }
}


/// A serde serializer that converts an enum variant into the variant's
/// name.
struct Serializer {}

impl<'a> SerdeSerializer for &'a mut Serializer {
  type Ok = &'static str;
  type Error = UnsupportedType;

  type SerializeSeq = Impossible<Self::Ok, Self::Error>;
  type SerializeTuple = Impossible<Self::Ok, Self::Error>;
  type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
  type SerializeTupleVariant = TupleVariantSerializer;
  type SerializeMap = Impossible<Self::Ok, Self::Error>;
  type SerializeStruct = Impossible<Self::Ok, Self::Error>;
  type SerializeStructVariant = StructVariantSerializer;

  #[inline]
  fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_bool"))
  }

  #[inline]
  fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_i8"))
  }

  #[inline]
  fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_i16"))
  }

  #[inline]
  fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_i32"))
  }

  #[inline]
  fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_i64"))
  }

  #[inline]
  fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_u8"))
  }

  #[inline]
  fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_u16"))
  }

  #[inline]
  fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_u32"))
  }

  #[inline]
  fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_u64"))
  }

  #[inline]
  fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_f32"))
  }

  #[inline]
  fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_f64"))
  }

  #[inline]
  fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_char"))
  }

  #[inline]
  fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_str"))
  }

  #[inline]
  fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_bytes"))
  }

  #[inline]
  fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_none"))
  }

  #[inline]
  fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
  where
    T: ?Sized + Serialize,
  {
    value.serialize(self)
  }

  #[inline]
  fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_unit"))
  }

  #[inline]
  fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_unit_struct"))
  }

  #[inline]
  fn serialize_unit_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
  ) -> Result<Self::Ok, Self::Error> {
    Ok(variant)
  }

  #[inline]
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

  #[inline]
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

  #[inline]
  fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
    Err(Self::Error::custom("serialize_seq"))
  }

  #[inline]
  fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
    Err(Self::Error::custom("serialize_tuple"))
  }

  #[inline]
  fn serialize_tuple_struct(
    self,
    _name: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleStruct, Self::Error> {
    Err(Self::Error::custom("serialize_tuple_struct"))
  }

  #[inline]
  fn serialize_tuple_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleVariant, Self::Error> {
    Ok(TupleVariantSerializer(variant))
  }

  #[inline]
  fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
    Err(Self::Error::custom("serialize_map"))
  }

  #[inline]
  fn serialize_struct(
    self,
    _name: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStruct, Self::Error> {
    Err(Self::Error::custom("serialize_struct"))
  }

  #[inline]
  fn serialize_struct_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStructVariant, Self::Error> {
    Ok(StructVariantSerializer(variant))
  }

  #[inline]
  fn collect_str<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
  where
    T: ?Sized,
  {
    Err(Self::Error::custom("collect_str"))
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn unit_variant_names() {
    #[derive(Serialize)]
    enum Foo {
      Var1,
      #[serde(rename = "VAR2")]
      Var2,
    }

    assert_eq!(to_variant_name(&Foo::Var1).unwrap(), "Var1");
    assert_eq!(to_variant_name(&Foo::Var2).unwrap(), "VAR2");
  }

  #[test]
  fn newtype_variant_names() {
    #[derive(Serialize)]
    enum Foo {
      Var1(()),
      #[serde(rename = "VAR2")]
      Var2(u32),
    }

    assert_eq!(to_variant_name(&Foo::Var1(())).unwrap(), "Var1");
    assert_eq!(to_variant_name(&Foo::Var2(42)).unwrap(), "VAR2");
  }

  #[test]
  fn tuple_variant_names() {
    #[derive(Serialize)]
    enum Foo {
      BAz((), u64),
      #[serde(rename = "VAR")]
      Var((), (), ()),
    }

    assert_eq!(to_variant_name(&Foo::BAz((), 1337)).unwrap(), "BAz");
    assert_eq!(to_variant_name(&Foo::Var((), (), ())).unwrap(), "VAR");
  }

  #[test]
  fn newtype_struct() {
    #[derive(Serialize)]
    struct Bar(u64);

    assert_eq!(to_variant_name(&Bar(42)).unwrap(), "Bar");
  }

  /// Check that we can correctly retrieve names of enum struct
  /// variants.
  #[test]
  fn struct_variant_names() {
    #[derive(Serialize)]
    enum Foo {
      Baz {
        i: i32,
      },
      #[serde(rename = "VAR")]
      Var {
        i: i32,
      },
    }

    assert_eq!(to_variant_name(&Foo::Baz { i: 0 }).unwrap(), "Baz");
    assert_eq!(to_variant_name(&Foo::Var { i: 0 }).unwrap(), "VAR");
  }
}
