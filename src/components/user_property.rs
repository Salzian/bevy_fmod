use bevy::prelude::Deref;
use std::ffi::CStr;

#[derive(Deref)]
pub struct UserProperty(libfmod::UserProperty);

impl UserProperty {
    pub(crate) fn get_value(&self) -> UserPropertyType {
        match self.type_ {
            libfmod::UserPropertyType::Integer => {
                let int_value = unsafe { self.union.intvalue };
                UserPropertyType::Int(int_value)
            }
            libfmod::UserPropertyType::Boolean => {
                let bool_value = unsafe { self.union.boolvalue } != 0;
                UserPropertyType::Bool(bool_value)
            }
            libfmod::UserPropertyType::Float => {
                let float_value = unsafe { self.union.floatvalue };
                UserPropertyType::Float(float_value)
            }
            libfmod::UserPropertyType::String => {
                let string_value = unsafe {
                    let ptr = self.union.stringvalue;
                    CStr::from_ptr(ptr)
                };
                UserPropertyType::String(
                    string_value
                        .to_str()
                        .expect("Failed to convert user property CStr to str.")
                        .to_string(),
                )
            }
        }
    }
}

impl From<libfmod::UserProperty> for UserProperty {
    fn from(value: libfmod::UserProperty) -> Self {
        Self(value)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum UserPropertyType {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
}
