use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

use crate::{quantity::Unit, Quantity};

impl<'de, const U: Unit> Deserialize<'de> for Quantity<U> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_f32(F32Visitor)
    }
}

struct F32Visitor<const U: Unit>;

impl<'de, const U: Unit> Visitor<'de> for F32Visitor<U> {
    type Value = Quantity<U>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a non-NaN floating point number")
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // TODO error
        Ok(Self::Value::new(value))
    }
}
