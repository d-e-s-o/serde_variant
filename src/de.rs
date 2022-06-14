use crate::error::Direction;
use crate::{Error, ErrorCode, Result};
use serde::de;
use serde::de::Error as DeError;

pub struct Deserializer<'de> {
    pub(crate) input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn new(input: &'de str) -> Self {
        Deserializer { input }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "bool".to_owned(),
        )))
    }

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "any".to_owned(),
        )))
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "i8".to_owned(),
        )))
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "i16".to_owned(),
        )))
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "i32".to_owned(),
        )))
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "i64".to_owned(),
        )))
    }

    fn deserialize_i128<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "i128".to_owned(),
        )))
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "u8".to_owned(),
        )))
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "u16".to_owned(),
        )))
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "u32".to_owned(),
        )))
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "u64".to_owned(),
        )))
    }

    fn deserialize_u128<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "u128".to_owned(),
        )))
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "f32".to_owned(),
        )))
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "f64".to_owned(),
        )))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "char".to_owned(),
        )))
    }

    fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = visitor.visit_borrowed_str(self.input);
        self.input = "";
        res
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "bytes".to_owned(),
        )))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "byte buf".to_owned(),
        )))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "tuple".to_owned(),
        )))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "struct".to_owned(),
        )))
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.input == name {
            self.input = "";
            visitor.visit_unit()
        } else {
            Err(ErrorCode::InvalidType {
                unexpected: self.input.to_owned(),
                expected: name.to_owned(),
            }
            .into())
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "tuple struct".to_owned(),
        )))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "map".to_owned(),
        )))
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "seq".to_owned(),
        )))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(VariantName::new(self))
    }

    fn is_human_readable(&self) -> bool {
        true
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "any".to_owned(),
        )))
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "new type struct".to_owned(),
        )))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let variant = self.input.to_owned();
        visitor
            .visit_enum(VariantName::new(self))
            .map_err(|_| Error::unknown_variant(&variant, variants))
    }
}

struct VariantName<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> VariantName<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        VariantName { de }
    }
}

impl<'de, 'a> de::EnumAccess<'de> for VariantName<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        Ok((seed.deserialize(&mut *self.de)?, self))
    }
}

impl<'de, 'a> de::VariantAccess<'de> for VariantName<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "new type variant".to_owned(),
        )))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "tuple variant".to_owned(),
        )))
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::from(ErrorCode::UnsupportedOperation(
            Direction::Deserialization,
            "struct variant".to_owned(),
        )))
    }
}
