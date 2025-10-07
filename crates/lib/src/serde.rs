#![cfg(feature = "serde")]
use crate::*;
use ::serde::*;
use std::str::FromStr;

// =====================
// === Serialization ===
// =====================

#[cfg(feature = "serde")]
impl Serialize for Dec19x19 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

// =======================
// === Deserialization ===
// =======================

impl<'de> Deserialize<'de> for Dec19x19 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        struct Visitor;

        impl de::Visitor<'_> for Visitor {
            type Value = Dec19x19;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "a string or number representing a fixed-point decimal with 19 fractional \
                    digits"
                )
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Dec19x19::from_str(v).map_err(E::custom)
            }

            fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
                Dec19x19::try_from(v).map_err(E::custom)
            }

            fn visit_i8<E: de::Error>(self, v: i8) -> Result<Self::Value, E> {
                Ok(Dec19x19::from(v))
            }

            fn visit_i16<E: de::Error>(self, v: i16) -> Result<Self::Value, E> {
                Ok(Dec19x19::from(v))
            }

            fn visit_i32<E: de::Error>(self, v: i32) -> Result<Self::Value, E> {
                Ok(Dec19x19::from(v))
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(Dec19x19::from(v))
            }

            fn visit_i128<E: de::Error>(self, v: i128) -> Result<Self::Value, E> {
                Dec19x19::try_from(v).map_err(E::custom)
            }

            fn visit_u8<E: de::Error>(self, v: u8) -> Result<Self::Value, E> {
                Ok(Dec19x19::from(v))
            }

            fn visit_u16<E: de::Error>(self, v: u16) -> Result<Self::Value, E> {
                Ok(Dec19x19::from(v))
            }

            fn visit_u32<E: de::Error>(self, v: u32) -> Result<Self::Value, E> {
                Ok(Dec19x19::from(v))
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                Dec19x19::try_from(v).map_err(E::custom)
            }

            fn visit_u128<E: de::Error>(self, v: u128) -> Result<Self::Value, E> {
                Dec19x19::try_from(v).map_err(E::custom)
            }

            #[cfg(feature = "serde_float")]
            fn visit_f32<E: de::Error>(self, v: f32) -> Result<Self::Value, E> {
                Dec19x19::try_from(v).map_err(E::custom)
            }

            #[cfg(feature = "serde_float")]
            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
                Dec19x19::try_from(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}
