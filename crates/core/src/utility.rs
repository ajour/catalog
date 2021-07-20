pub mod null_to_default {
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: Default + Deserialize<'de>,
    {
        let opt = Option::deserialize(deserializer)?;
        Ok(opt.unwrap_or_default())
    }
}

pub mod number_and_string_to_i32 {
    use serde::{self, de, Deserialize, Deserializer};
    use std::convert::TryFrom;

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i32, D::Error> {
        Ok(match serde_json::Value::deserialize(deserializer)? {
            serde_json::Value::String(s) => s.parse().map_err(de::Error::custom)?,
            serde_json::Value::Number(num) => {
                let num = num
                    .as_i64()
                    .ok_or_else(|| de::Error::custom(format!("invalid number: {}", num)))?;
                i32::try_from(num).ok().unwrap_or(0)
            }
            _ => return Err(de::Error::custom("wrong type")),
        })
    }
}

pub mod number_and_string_to_u64 {
    use serde::{self, de, Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        Ok(match serde_json::Value::deserialize(deserializer)? {
            serde_json::Value::String(s) => s.parse().map_err(de::Error::custom)?,
            serde_json::Value::Number(num) => num
                .as_u64()
                .ok_or_else(|| de::Error::custom(format!("Invalid number: {}", num)))?,
            _ => return Err(de::Error::custom("wrong type")),
        })
    }
}

pub mod u64_to_string {
    use serde::{self, de, Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
        Ok(match serde_json::Value::deserialize(deserializer)? {
            serde_json::Value::Number(num) => {
                let num = num
                    .as_u64()
                    .ok_or_else(|| de::Error::custom(format!("invalid number: {}", num)))?;
                num.to_string()
            }
            _ => return Err(de::Error::custom("wrong type")),
        })
    }
}
