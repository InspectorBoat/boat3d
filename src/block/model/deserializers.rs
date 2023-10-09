use crate::{block::model::deserialize::MaybeDeserialize, cull::math::W};
use std::{mem, collections::HashMap};

use cgmath::{Vector3, Zero};
use json::JsonValue;

use super::deserialize::{Deserialize, JsonDeserializeError::{self, *}, JsonType::{self, *}};

impl Deserialize for Vector3<f64> {
    fn deserialize(value: &json::JsonValue) -> Result<Self, JsonDeserializeError> { unsafe {
        if let JsonValue::Object(object) = value {
            let x = f64::deserialize_field(object, "x");
            let y = f64::deserialize_field(object, "y");
            let z = f64::deserialize_field(object, "z");

            let Ok(x) = x else { return Err(x.unwrap_err_unchecked()); };
            let Ok(y) = y else { return Err(y.unwrap_err_unchecked()); };
            let Ok(z) = z else { return Err(z.unwrap_err_unchecked()); };

            return Ok(Vector3 {
                x: x,
                y: y,
                z: z,
            });
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    } }
}

impl <const N: usize> Deserialize for [f64; N] {
    fn deserialize(value: &json::JsonValue) -> Result<Self, JsonDeserializeError> { unsafe {
        if let JsonValue::Array(array) = value {
            let mut result: [f64; N] = mem::zeroed();
            if array.len() != N {
                return Err(WrongLength(array.len()));
            }
            for i in 0..N {
                let elem = f64::deserialize(&array[i]);
                if let Ok(num) = elem {
                    result[i] = num;
                } else if let Err(err) = elem {
                    return Err(err);
                }
            }
            return Ok(result);
        } else {
            return Err(WrongType(JsonType::get_type(value)))
        }
    } }
}

impl <T> Deserialize for HashMap<std::string::String, T> where T: Deserialize {
    fn deserialize(value: &JsonValue) -> Result<Self, JsonDeserializeError> {
        if let JsonValue::Object(object) = value {
            let mut result = HashMap::new();
            for (key, value) in object.iter() {
                let value = T::deserialize(value);
                if let Ok(value) = value {
                    result.insert(key.to_string(), value);
                } else if let Err(err) = value {
                    return Err(err);
                }
            }
            return Ok(result);
        } else {
            return Err(WrongType(JsonType::get_type(value)))
        }
    }
}

impl <T> Deserialize for Vec<T> where T: Deserialize {
    fn deserialize(value: &JsonValue) -> Result<Self, JsonDeserializeError> {
        if let JsonValue::Array(array) = value {
            let mut result = Vec::new();
            for i in 0..array.len() {
                let value = T::deserialize(&array[i]);
                if let Ok(value) = value {
                    result.push(value);
                } else if let Err(err) = value {
                    return Err(err);
                }
            }
            return Ok(result);
        } else {
            return Err(WrongType(JsonType::get_type(value)))
        }
    }
}

impl Deserialize for f64 {
    fn deserialize(value: &json::JsonValue) -> Result<Self, JsonDeserializeError> {
        if let JsonValue::Number(number) = value {
            return Ok((*number).into());
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    }
}

impl Deserialize for u64 {
    fn deserialize(value: &json::JsonValue) -> Result<Self, JsonDeserializeError> {
        if let JsonValue::Number(number) = value {
            return Ok((*number).as_fixed_point_u64(0).unwrap());
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    }
}

impl Deserialize for bool {
    fn deserialize(value: &JsonValue) -> Result<Self, JsonDeserializeError> {
        if let JsonValue::Boolean(boolean) = value {
            return Ok(*boolean);
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    }
}

impl Deserialize for std::string::String {
    fn deserialize(value: &JsonValue) -> Result<Self, JsonDeserializeError> {
        if let JsonValue::String(string) = value {
            return Ok(string.to_string());
        } else if let JsonValue::Short(short) = value {
            return Ok(short.to_string());
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    }
}