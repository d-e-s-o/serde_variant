use crate::error::Direction;
use crate::{Error, ErrorCode, Result};
use serde::ser;
use serde::Serialize;

/// A serializer for tuple enum variants.
pub struct TupleVariantSerializer(&'static str);

impl ser::SerializeTupleVariant for TupleVariantSerializer {
    type Ok = &'static str;
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.0)
    }
}

/// A serializer for struct enum variants.
pub struct StructVariantSerializer(&'static str);

impl ser::SerializeStructVariant for StructVariantSerializer {
    type Ok = &'static str;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.0)
    }
}

/// A serde serializer that converts an enum variant into the variant's
/// name.
pub struct Serializer {}

type SerializationResult = Result<&'static str>;

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = &'static str;
    type Error = Error;

    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = TupleVariantSerializer;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = StructVariantSerializer;

    fn serialize_bool(self, _v: bool) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "bool".to_owned(),
        )))
    }

    fn serialize_i8(self, _v: i8) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "i8".to_owned(),
        )))
    }

    fn serialize_i16(self, _v: i16) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "i16".to_owned(),
        )))
    }

    fn serialize_i32(self, _v: i32) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "i32".to_owned(),
        )))
    }

    fn serialize_i64(self, _v: i64) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "i64".to_owned(),
        )))
    }

    fn serialize_u8(self, _v: u8) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "u8".to_owned(),
        )))
    }

    fn serialize_u16(self, _v: u16) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "u16".to_owned(),
        )))
    }

    fn serialize_u32(self, _v: u32) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "u32".to_owned(),
        )))
    }

    fn serialize_u64(self, _v: u64) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "u64".to_owned(),
        )))
    }

    fn serialize_f32(self, _v: f32) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "f32".to_owned(),
        )))
    }

    fn serialize_f64(self, _v: f64) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "f64".to_owned(),
        )))
    }

    fn serialize_char(self, _v: char) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "char".to_owned(),
        )))
    }

    fn serialize_str(self, _v: &str) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "str".to_owned(),
        )))
    }

    fn serialize_bytes(self, _v: &[u8]) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "bytes".to_owned(),
        )))
    }

    fn serialize_none(self) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "none".to_owned(),
        )))
    }

    fn serialize_some<T>(self, value: &T) -> SerializationResult
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> SerializationResult {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "unit".to_owned(),
        )))
    }

    fn serialize_unit_struct(self, name: &'static str) -> SerializationResult {
        Ok(name)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "seq".to_owned(),
        )))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "tuple".to_owned(),
        )))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "tuple struct".to_owned(),
        )))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(TupleVariantSerializer(variant))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "map".to_owned(),
        )))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "struct".to_owned(),
        )))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(StructVariantSerializer(variant))
    }

    fn collect_str<T>(self, _value: &T) -> SerializationResult
    where
        T: ?Sized,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            "collect str".to_owned(),
        )))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> SerializationResult {
        Ok(variant)
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, _value: &T) -> SerializationResult
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
    ) -> SerializationResult
    where
        T: ?Sized + Serialize,
    {
        Ok(variant)
    }
}
