use serde::{Deserialize, Serialize};

use super::*;

impl Serialize for s32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.0)
    }
}

impl<'de> Deserialize<'de> for s32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let i32 = i32::deserialize(deserializer)?;
        Ok(s32(i32))
    }
}
