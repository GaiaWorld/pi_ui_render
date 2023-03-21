use json::JsonValue;
use ordered_float::NotNan;
use pi_atom::Atom;

pub trait FromJsonValue<'a>
{
	type Output: Sized;
    fn from(json_value: &'a JsonValue) -> Option<Self::Output>;
}

impl<'a> FromJsonValue<'a> for String {
	type Output = Self;
    fn from(json_value: &'a JsonValue) -> Option<Self> {
        match json_value.as_str() {
            Some(r) => Some(r.to_string()),
            None => None,
        }
    }
}

impl<'a> FromJsonValue<'a> for bool {
	type Output = Self;
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_bool() }
}

impl<'a> FromJsonValue<'a> for Atom {
	type Output = Self;
    fn from(json_value: &'a JsonValue) -> Option<Self> {
        match json_value.as_str() {
            Some(r) => Some(<Atom as From<&str>>::from(r)),
            None => None,
        }
    }
}

impl<'a> FromJsonValue<'a> for &'a str {
	type Output = Self;
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_str() }
}
impl<'a> FromJsonValue<'a> for str {
	type Output = String;
    fn from(json_value: &'a JsonValue) -> Option<String> { json_value.as_str().map(|r| r.to_string()) }
}
impl<'a> FromJsonValue<'a> for [u8] {
	type Output = Vec<u8>;
    fn from(json_value: &'a JsonValue) -> Option<Vec<u8>> { <Vec<u8> as FromJsonValue>::from(json_value) }
}
impl<'a> FromJsonValue<'a> for f32 {
	type Output = Self;
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_f32() }
}

impl<'a> FromJsonValue<'a> for f64 {
	type Output = Self;
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_f64() }
}

impl<'a> FromJsonValue<'a> for usize {
	type Output = Self;
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_usize() }
}

impl<'a> FromJsonValue<'a> for u32 {
	type Output = Self;
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_u32() }
}

impl<'a> FromJsonValue<'a> for i32 {
	type Output = Self;
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_i32() }
}

impl<'a> FromJsonValue<'a> for u8 {
	type Output = Self;
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_u8() }
}

impl<'a> FromJsonValue<'a> for isize {
	type Output = Self;
    fn from(json_value: &'a JsonValue) -> Option<Self> { json_value.as_isize() }
}

impl<'a> FromJsonValue<'a> for NotNan<f32> {
	type Output = Self;
    fn from(json_value: &'a JsonValue) -> Option<Self> {
        match json_value.as_f32() {
            Some(r) => Some(unsafe { NotNan::new_unchecked(r) }),
            None => None,
        }
    }
}

pub fn as_value<'a, T: FromJsonValue<'a> + ?Sized>(json_value: &'a [JsonValue], i: usize) -> Option<T::Output> { T::from(&json_value[i]) }

impl<'a, T: FromJsonValue<'a>> FromJsonValue<'a> for Vec<T> {
	type Output = Vec<T::Output>;
    fn from(json_value: &'a JsonValue) -> Option<Self::Output> {
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

