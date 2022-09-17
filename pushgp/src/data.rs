use get_size::GetSize;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use smartstring::{LazyCompact, SmartString};

use crate::{Code, Float, Name};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Data {
    /// Indicates the Code has no data associated with it. This is slightly preferrable to Option<Data> because wrapping
    /// in an Option forces Code to have two enums and thus larger size.
    None,
    Integer(i64),
    UnsignedInteger(u64),
    Decimal(Decimal),

    /// Use for Names. The extract_names function looks for this type specifically
    Name(Name),

    /// The SmartString will keep strings smaller than 24 bytes on the stack and avoid heap allocations
    String(SmartString<LazyCompact>),

    /// If a string is known to be static, we can pass the pointer efficiently
    StaticString(&'static str),

    /// Use this variant to encode custom data that doesn't fit nicely into any other category but is small enough to
    /// be stored on the stack. At 30 bytes it does not increase memory usage for Data
    StackBytes([u8; 30]),

    /// Use this variant to encode custom data that doesn't fit nicely into any other category and is larger than
    /// 30 bytes
    Bytes(Vec<u8>),

    /// Holds the data for a list
    CodeList(Vec<Code>),
}

impl Data {
    pub fn bool_value(&self) -> Option<bool> {
        match self {
            Data::Integer(x) => Some(*x != 0),
            _ => None,
        }
    }

    pub fn integer_value(&self) -> Option<i64> {
        match self {
            Data::Integer(x) => Some(*x),
            _ => None,
        }
    }

    pub fn unsigned_integer_value(&self) -> Option<u64> {
        match self {
            Data::UnsignedInteger(x) => Some(*x),
            _ => None,
        }
    }

    pub fn decimal_value(&self) -> Option<Decimal> {
        match self {
            Data::Decimal(x) => Some(*x),
            _ => None,
        }
    }

    pub fn name_value(&self) -> Option<Name> {
        match self {
            Data::Name(x) => Some(x.clone()),
            Data::String(x) => Some(x.clone().into()),
            _ => None,
        }
    }

    pub fn string_value(&self) -> Option<String> {
        match self {
            Data::Name(x) => Some(x.to_string()),
            Data::String(x) => Some(x.clone().into()),
            _ => None,
        }
    }

    pub fn static_string_value(&self) -> Option<&'static str> {
        match self {
            Data::StaticString(x) => Some(x),
            _ => None,
        }
    }

    pub fn code_iter(&self) -> Option<std::slice::Iter<'_, Code>> {
        match self {
            Data::CodeList(list) => Some(list.iter()),
            _ => None,
        }
    }
}

const SIZE_OF_VEC_U8: usize = std::mem::size_of::<Vec<u8>>();

impl GetSize for Data {
    fn get_heap_size(&self) -> usize {
        match self {
            // These two have heap data equivilant to length of the string if it is more than the size of a Vec<u8>
            Data::Name(name) => {
                if name.len() < SIZE_OF_VEC_U8 {
                    0
                } else {
                    name.len()
                }
            }
            Data::String(string) => {
                if string.len() < SIZE_OF_VEC_U8 {
                    0
                } else {
                    string.len()
                }
            }

            // The heap size is the capacity of the vector
            Data::Bytes(bytes) => bytes.capacity(),

            // The heap size is the sum of the size (stack + heap) of all items because they are all stored on the heap
            Data::CodeList(list) => list.iter().map(|item| item.get_size()).sum(),

            // All other variants have no heap data
            _ => 0,
        }
    }
}

impl From<bool> for Data {
    fn from(value: bool) -> Self {
        Data::Integer(if value { 1 } else { 0 })
    }
}

impl From<i8> for Data {
    fn from(value: i8) -> Self {
        Data::Integer(value as i64)
    }
}

impl From<i16> for Data {
    fn from(value: i16) -> Self {
        Data::Integer(value as i64)
    }
}

impl From<i32> for Data {
    fn from(value: i32) -> Self {
        Data::Integer(value as i64)
    }
}

impl From<i64> for Data {
    fn from(value: i64) -> Self {
        Data::Integer(value as i64)
    }
}

impl From<isize> for Data {
    fn from(value: isize) -> Self {
        Data::Integer(value as i64)
    }
}

impl From<u8> for Data {
    fn from(value: u8) -> Self {
        Data::UnsignedInteger(value as u64)
    }
}

impl From<u16> for Data {
    fn from(value: u16) -> Self {
        Data::UnsignedInteger(value as u64)
    }
}

impl From<u32> for Data {
    fn from(value: u32) -> Self {
        Data::UnsignedInteger(value as u64)
    }
}

impl From<u64> for Data {
    fn from(value: u64) -> Self {
        Data::UnsignedInteger(value as u64)
    }
}

impl From<usize> for Data {
    fn from(value: usize) -> Self {
        Data::UnsignedInteger(value as u64)
    }
}

impl From<f32> for Data {
    fn from(value: f32) -> Self {
        Data::Decimal(Decimal::from_f32(value).unwrap())
    }
}

impl From<f64> for Data {
    fn from(value: f64) -> Self {
        Data::Decimal(Decimal::from_f64(value).unwrap())
    }
}

impl From<Float> for Data {
    fn from(value: Float) -> Self {
        Data::Decimal(value.into())
    }
}

impl From<Decimal> for Data {
    fn from(value: Decimal) -> Self {
        Data::Decimal(value)
    }
}

impl From<Name> for Data {
    fn from(value: Name) -> Self {
        Data::Name(value)
    }
}

impl From<Vec<Code>> for Data {
    fn from(list: Vec<Code>) -> Self {
        Data::CodeList(list)
    }
}
