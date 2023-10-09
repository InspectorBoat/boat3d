use crate::block::model::deserialize::JsonDeserializeError::MissingField;
use json::{JsonValue, object::Object};

pub trait Deserialize where Self: Sized {
    fn deserialize(value: &JsonValue) -> Result<Self, JsonDeserializeError>;
    fn deserialize_field(parent: &Object, field: &str) -> Result<Self, JsonDeserializeError> {
        if let Some(value) = parent.get(field) {
            return Self::deserialize(value);
        } else {
            return Err(MissingField(field.to_string()));
        }
    }
}

pub trait MaybeDeserialize where Self: Sized + Deserialize {
    fn maybe_deserialize(value: Option<&JsonValue>) -> Result<Option<Self>, JsonDeserializeError>;
    fn maybe_deserialize_field(parent: &Object, field: &str) -> Result<Option<Self>, JsonDeserializeError> { unsafe {
        if let Some(value) = parent.get(field) {
            let result = Self::deserialize(value);
            if let Ok(value) = result {
                return Ok(Some(value));
            } else {
                return Err(result.unwrap_err_unchecked());
            }
        } else {
            return Ok(None);
        }
    }
} }

impl <T: Deserialize> MaybeDeserialize for T {
    fn maybe_deserialize(value: Option<&JsonValue>) -> Result<Option<Self>, JsonDeserializeError> { unsafe {
        match value {
            None => { return Ok(None); }
            Some(value) => {
                let result = T::deserialize(value);
                if let Ok(value) = result {
                    return Ok(Some(value))
                } else {
                    return Err(result.unwrap_err_unchecked());
                }
            }
        }
    } }
}

#[derive(Debug)]
pub enum JsonDeserializeError {
    WrongType(JsonType), // actual type
    MissingField(String), // missing field
    WrongLength(usize), // actual size
    BadStringEnum(String)
}

#[derive(Debug)]
pub enum JsonType {
    Null,
    Short,
    String,
    Number,
    Boolean,
    Object,
    Array,
}

impl JsonType {
    pub fn get_type(value: &JsonValue) -> JsonType {
        match value {
            JsonValue::Null => { return JsonType::Null; }
            JsonValue::Short(_) => { return JsonType::Short; }
            JsonValue::String(_) => { return JsonType::String; }
            JsonValue::Number(_) => { return JsonType::Number; }
            JsonValue::Boolean(_) => { return JsonType::Boolean; }
            JsonValue::Object(_) => { return JsonType::Object; }
            JsonValue::Array(_) => { return JsonType::Array; }
        }
    }
}