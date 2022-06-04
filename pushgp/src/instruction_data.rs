use crate::Code;
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InstructionData {
    Bool(bool),
    U128(u128),
    I128(i128),
    Float(Decimal),
    String(String),
    Code(Box<Code>),
}

impl InstructionData {
    pub fn from_bool(b: bool) -> InstructionData {
        InstructionData::Bool(b)
    }

    pub fn from_u8(u: u8) -> InstructionData {
        InstructionData::U128(u as u128)
    }

    pub fn from_u16(u: u16) -> InstructionData {
        InstructionData::U128(u as u128)
    }

    pub fn from_u32(u: u32) -> InstructionData {
        InstructionData::U128(u as u128)
    }

    pub fn from_u64(u: u64) -> InstructionData {
        InstructionData::U128(u as u128)
    }

    pub fn from_u128(u: u128) -> InstructionData {
        InstructionData::U128(u)
    }

    pub fn from_usize(u: usize) -> InstructionData {
        InstructionData::U128(u as u128)
    }

    pub fn from_i8(i: i8) -> InstructionData {
        InstructionData::I128(i as i128)
    }

    pub fn from_i16(i: i16) -> InstructionData {
        InstructionData::I128(i as i128)
    }

    pub fn from_i32(i: i32) -> InstructionData {
        InstructionData::I128(i as i128)
    }

    pub fn from_i64(i: i64) -> InstructionData {
        InstructionData::I128(i as i128)
    }

    pub fn from_i128(i: i128) -> InstructionData {
        InstructionData::I128(i)
    }

    pub fn from_isize(i: isize) -> InstructionData {
        InstructionData::I128(i as i128)
    }

    pub fn from_f32(f: f32) -> InstructionData {
        InstructionData::Float(Decimal::from_f32(f).unwrap())
    }

    pub fn from_f64(f: f64) -> InstructionData {
        InstructionData::Float(Decimal::from_f64(f).unwrap())
    }

    pub fn from_string<S: Into<String>>(s: S) -> InstructionData {
        InstructionData::String(s.into())
    }

    pub fn from_code(c: Code) -> InstructionData {
        InstructionData::Code(Box::new(c))
    }

    pub fn get_bool(&self) -> Option<bool> {
        if let InstructionData::Bool(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn get_u8(&self) -> Option<u8> {
        if let InstructionData::U128(v) = self {
            Some(*v as u8)
        } else {
            None
        }
    }

    pub fn get_u16(&self) -> Option<u16> {
        if let InstructionData::U128(v) = self {
            Some(*v as u16)
        } else {
            None
        }
    }

    pub fn get_u32(&self) -> Option<u32> {
        if let InstructionData::U128(v) = self {
            Some(*v as u32)
        } else {
            None
        }
    }

    pub fn get_u64(&self) -> Option<u64> {
        if let InstructionData::U128(v) = self {
            Some(*v as u64)
        } else {
            None
        }
    }

    pub fn get_u128(&self) -> Option<u128> {
        if let InstructionData::U128(v) = self {
            Some(*v as u128)
        } else {
            None
        }
    }

    pub fn get_usize(&self) -> Option<usize> {
        if let InstructionData::U128(v) = self {
            Some(*v as usize)
        } else {
            None
        }
    }

    pub fn get_i8(&self) -> Option<i8> {
        if let InstructionData::I128(v) = self {
            Some(*v as i8)
        } else {
            None
        }
    }

    pub fn get_i16(&self) -> Option<i16> {
        if let InstructionData::I128(v) = self {
            Some(*v as i16)
        } else {
            None
        }
    }

    pub fn get_i32(&self) -> Option<i32> {
        if let InstructionData::I128(v) = self {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn get_i64(&self) -> Option<i64> {
        if let InstructionData::I128(v) = self {
            Some(*v as i64)
        } else {
            None
        }
    }

    pub fn get_i128(&self) -> Option<i128> {
        if let InstructionData::I128(v) = self {
            Some(*v as i128)
        } else {
            None
        }
    }

    pub fn get_isize(&self) -> Option<isize> {
        if let InstructionData::I128(v) = self {
            Some(*v as isize)
        } else {
            None
        }
    }

    pub fn get_f32(&self) -> Option<f32> {
        if let InstructionData::Float(v) = self {
            Some((*v).to_f32().unwrap())
        } else {
            None
        }
    }

    pub fn get_f64(&self) -> Option<f64> {
        if let InstructionData::Float(v) = self {
            Some((*v).to_f64().unwrap())
        } else {
            None
        }
    }

    pub fn get_decimal(&self) -> Option<Decimal> {
        if let InstructionData::Float(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn get_string(&self) -> Option<String> {
        if let InstructionData::String(v) = self {
            Some(v.clone())
        } else {
            None
        }
    }

    pub fn get_code(&self) -> Option<Code> {
        if let InstructionData::Code(v) = self {
            Some(*v.clone())
        } else {
            None
        }
    }
}
