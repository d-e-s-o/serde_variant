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

/// A serde serializer that converts an enum or struct into its name.
pub struct Serializer {}

type SerializationResult = Result<&'static str>;

macro_rules! unsupported {
    ($ty:expr) => {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Serialization,
            $ty.to_owned(),
        )))
    };
}

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
        unsupported!("bool")
    }

    fn serialize_i8(self, _v: i8) -> SerializationResult {
        unsupported!("i8")
    }

    fn serialize_i16(self, _v: i16) -> SerializationResult {
        unsupported!("i16")
    }

    fn serialize_i32(self, _v: i32) -> SerializationResult {
        unsupported!("i32")
    }

    fn serialize_i64(self, _v: i64) -> SerializationResult {
        unsupported!("i64")
    }

    fn serialize_u8(self, _v: u8) -> SerializationResult {
        unsupported!("u8")
    }

    fn serialize_u16(self, _v: u16) -> SerializationResult {
        unsupported!("u16")
    }

    fn serialize_u32(self, _v: u32) -> SerializationResult {
        unsupported!("u32")
    }

    fn serialize_u64(self, _v: u64) -> SerializationResult {
        unsupported!("u64")
    }

    fn serialize_f32(self, _v: f32) -> SerializationResult {
        unsupported!("f32")
    }

    fn serialize_f64(self, _v: f64) -> SerializationResult {
        unsupported!("f64")
    }

    fn serialize_char(self, _v: char) -> SerializationResult {
        unsupported!("char")
    }

    fn serialize_str(self, _v: &str) -> SerializationResult {
        unsupported!("str")
    }

    fn serialize_bytes(self, _v: &[u8]) -> SerializationResult {
        unsupported!("bytes")
    }

    fn serialize_none(self) -> SerializationResult {
        unsupported!("none")
    }

    fn serialize_some<T>(self, value: &T) -> SerializationResult
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> SerializationResult {
        unsupported!("unit")
    }

    fn serialize_unit_struct(self, name: &'static str) -> SerializationResult {
        Ok(name)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        unsupported!("seq")
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unsupported!("tuple")
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unsupported!("tuple struct")
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
        unsupported!("map")
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        unsupported!("struct")
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
        unsupported!("collect str")
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
