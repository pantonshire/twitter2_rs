use std::{fmt, str};

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct IdU64(pub(crate) u64);

impl Serialize for IdU64 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = self.0.to_string();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for IdU64 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(IdU64Visitor)
    }
}

struct IdU64Visitor;

impl IdU64Visitor {
    fn try_visit_int<E, I>(self, v: I) -> Result<IdU64, E>
    where
        E: Error,
        u64: TryFrom<I>,
        I: Copy + fmt::Display,
    {
        u64::try_from(v)
            .map_err(|_| E::custom(format!("id out of range: {}", v)))
            .and_then(|v| self.visit_u64(v))
    }
}

impl<'de> Visitor<'de> for IdU64Visitor {
    type Value = IdU64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an unsigned 64-bit integer id")
    }

    fn visit_i8<E: Error>(self, v: i8) -> Result<Self::Value, E> {
        self.try_visit_int(v)
    }

    fn visit_i16<E: Error>(self, v: i16) -> Result<Self::Value, E> {
        self.try_visit_int(v)
    }

    fn visit_i32<E: Error>(self, v: i32) -> Result<Self::Value, E> {
        self.try_visit_int(v)
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        self.try_visit_int(v)
    }

    fn visit_i128<E: Error>(self, v: i128) -> Result<Self::Value, E> {
        self.try_visit_int(v)
    }

    fn visit_u8<E: Error>(self, v: u8) -> Result<Self::Value, E> {
        self.visit_u64(u64::from(v))
    }

    fn visit_u16<E: Error>(self, v: u16) -> Result<Self::Value, E> {
        self.visit_u64(u64::from(v))
    }

    fn visit_u32<E: Error>(self, v: u32) -> Result<Self::Value, E> {
        self.visit_u64(u64::from(v))
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(IdU64(v))
    }

    fn visit_u128<E: Error>(self, v: u128) -> Result<Self::Value, E> {
        self.try_visit_int(v)
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        v.parse()
            .map_err(|_| E::custom(format!("id is not a valid u64: \"{}\"", v)))
            .and_then(|v| self.visit_u64(v))
    }
}
