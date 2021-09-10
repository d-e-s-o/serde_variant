// Copyright (C) 2020-2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use serde::ser::Error as SerdeError;
use serde::ser::Impossible;
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
  fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
    write!(fmt, "{} is not supported", self.method)
  }
}

impl Error for UnsupportedType {}

impl SerdeError for UnsupportedType {
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
pub fn to_variant_name<T>(value: &T) -> Result<&'static str, UnsupportedType>
where
  T: Serialize,
{
  let mut serializer = Serializer {};
  value.serialize(&mut serializer)
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
  type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
  type SerializeMap = Impossible<Self::Ok, Self::Error>;
  type SerializeStruct = Impossible<Self::Ok, Self::Error>;
  type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

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

  fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
    Err(Self::Error::custom("serialize_unit_struct"))
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
    _variant: &'static str,
    _value: &T,
  ) -> Result<Self::Ok, Self::Error>
  where
    T: ?Sized + Serialize,
  {
    Err(Self::Error::custom("serialize_newtype_variant"))
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
    _variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleVariant, Self::Error> {
    Err(Self::Error::custom("serialize_tuple_variant"))
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
    _variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStructVariant, Self::Error> {
    Err(Self::Error::custom("serialize_struct_variant"))
  }

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

  #[derive(Serialize)]
  enum Foo {
    Var1,
    #[serde(rename = "VAR2")]
    Var2,
  }

  #[test]
  fn unit_variant_names() {
    assert_eq!(to_variant_name(&Foo::Var1).unwrap(), "Var1");
    assert_eq!(to_variant_name(&Foo::Var2).unwrap(), "VAR2");
  }

  #[derive(Serialize)]
  struct Bar(u64);

  #[test]
  fn newtype_struct() {
    assert_eq!(to_variant_name(&Bar(42)).unwrap(), "Bar");
  }
}
