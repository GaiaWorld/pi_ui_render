use json::JsonValue;
use ordered_float::NotNan;
use pi_atom::Atom;

pub trait FromJsonValue<'a>
where
    Self: Sized,
{
    fn from(json_value: &'a JsonValue) -> Option<Self>;
}

impl<'a> FromJsonValue<'a> for String {
    fn from(json_value: &'a JsonValue) -> Option<Self> {
        match json_value.as_str() {
            Some(r) => Some(r.to_string()),
            None => None,
        }
    }
}

impl<'a> FromJsonValue<'a> for bool {
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_bool() }
}

impl<'a> FromJsonValue<'a> for Atom {
    fn from(json_value: &'a JsonValue) -> Option<Self> {
        match json_value.as_str() {
            Some(r) => Some(<Atom as From<&str>>::from(r)),
            None => None,
        }
    }
}

impl<'a> FromJsonValue<'a> for &'a str {
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_str() }
}

impl<'a> FromJsonValue<'a> for f32 {
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_f32() }
}

impl<'a> FromJsonValue<'a> for f64 {
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_f64() }
}

impl<'a> FromJsonValue<'a> for usize {
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_usize() }
}

impl<'a> FromJsonValue<'a> for u32 {
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_u32() }
}

impl<'a> FromJsonValue<'a> for i32 {
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_i32() }
}

impl<'a> FromJsonValue<'a> for u8 {
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_u8() }
}

impl<'a> FromJsonValue<'a> for isize {
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_isize() }
}

impl<'a> FromJsonValue<'a> for NotNan<f32> {
    fn from(json_value: &'a JsonValue) -> Option<Self> {
        match json_value.as_f32() {
            Some(r) => Some(unsafe { NotNan::new_unchecked(r) }),
            None => None,
        }
    }
}

pub fn as_value<'a, T: FromJsonValue<'a>>(json_value: &'a [JsonValue], i: usize) -> Option<T> { T::from(&json_value[i]) }

impl<'a, T: FromJsonValue<'a>> FromJsonValue<'a> for Vec<T> {
    fn from(json_value: &'a JsonValue) -> Option<Self> {
        // {
        // 	"ty": "Uint32Array",
        // 	"value": [3069761967]
        // }
        if let JsonValue::Object(r) = json_value {
            let value = match r.get("value") {
                Some(r) => r,
                None => return None,
            };
            match value {
                JsonValue::Array(r) => {
                    let mut vec = Vec::new();
                    for i in r.iter() {
                        vec.push(match T::from(i) {
                            Some(r) => r,
                            None => return None,
                        })
                    }
                    Some(vec)
                }
                _ => None,
            }
        } else {
            return None;
        }
    }
}
