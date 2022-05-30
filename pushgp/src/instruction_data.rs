use std::mem::ManuallyDrop;
use std::ops::Deref;
use ordered_float::OrderedFloat;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum DataType {
    Bool,
    U128,
    I128,
    Float,
    String,
}

union DataUnion {
    b: bool,
    u: u128,
    i: i128,
    f: OrderedFloat<f64>,
    s: ManuallyDrop<String>,
}

//#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InstructionData {
    data_type: DataType,
    data_union: DataUnion,
}

impl InstructionData {
    pub fn from_bool(b: bool) -> InstructionData {
        InstructionData { data_type: DataType::Bool, data_union: DataUnion { b } }
    }

    pub fn from_u8(u: u8) -> InstructionData {
        InstructionData { data_type: DataType::U128, data_union: DataUnion { u: u as u128 } }
    }

    pub fn from_u16(u: u16) -> InstructionData {
        InstructionData { data_type: DataType::U128, data_union: DataUnion { u: u as u128 } }
    }

    pub fn from_u32(u: u32) -> InstructionData {
        InstructionData { data_type: DataType::U128, data_union: DataUnion { u: u as u128 } }
    }

    pub fn from_u64(u: u64) -> InstructionData {
        InstructionData { data_type: DataType::U128, data_union: DataUnion { u: u as u128 } }
    }

    pub fn from_u128(u: u128) -> InstructionData {
        InstructionData { data_type: DataType::U128, data_union: DataUnion { u } }
    }

    pub fn from_usize(u: usize) -> InstructionData {
        InstructionData { data_type: DataType::U128, data_union: DataUnion { u: u as u128 } }
    }

    pub fn from_i8(i: i8) -> InstructionData {
        InstructionData { data_type: DataType::I128, data_union: DataUnion { i: i as i128 } }
    }

    pub fn from_i16(i: i16) -> InstructionData {
        InstructionData { data_type: DataType::I128, data_union: DataUnion { i: i as i128 } }
    }

    pub fn from_i32(i: i32) -> InstructionData {
        InstructionData { data_type: DataType::I128, data_union: DataUnion { i: i as i128 } }
    }

    pub fn from_i64(i: i64) -> InstructionData {
        InstructionData { data_type: DataType::I128, data_union: DataUnion { i: i as i128 } }
    }

    pub fn from_i128(i: i128) -> InstructionData {
        InstructionData { data_type: DataType::I128, data_union: DataUnion { i } }
    }

    pub fn from_isize(i: isize) -> InstructionData {
        InstructionData { data_type: DataType::I128, data_union: DataUnion { i: i as i128 } }
    }

    pub fn from_f32(f: f32) -> InstructionData {
        InstructionData { data_type: DataType::Float, data_union: DataUnion { f: OrderedFloat(f as f64) } }
    }

    pub fn from_f64(f: f64) -> InstructionData {
        InstructionData { data_type: DataType::Float, data_union: DataUnion { f: OrderedFloat(f as f64) } }
    }

    pub fn from_string<S: Into<String>>(s: S) -> InstructionData {
        InstructionData { data_type: DataType::String, data_union: DataUnion { s: ManuallyDrop::new(s.into()) } }
    }

    pub fn get_bool(&self) -> Option<bool> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::Bool, data_union: DataUnion { b } } => Some(*b),
                _ => None,
            }
        }
    }

    pub fn get_u8(&self) -> Option<u8> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::U128, data_union: DataUnion { u } } => Some(*u as u8),
                _ => None,
            }
        }
    }

    pub fn get_u16(&self) -> Option<u16> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::U128, data_union: DataUnion { u } } => Some(*u as u16),
                _ => None,
            }
        }
    }

    pub fn get_u32(&self) -> Option<u32> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::U128, data_union: DataUnion { u } } => Some(*u as u32),
                _ => None,
            }
        }
    }

    pub fn get_u64(&self) -> Option<u64> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::U128, data_union: DataUnion { u } } => Some(*u as u64),
                _ => None,
            }
        }
    }

    pub fn get_u128(&self) -> Option<u128> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::U128, data_union: DataUnion { u } } => Some(*u as u128),
                _ => None,
            }
        }
    }

    pub fn get_usize(&self) -> Option<usize> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::U128, data_union: DataUnion { u } } => Some(*u as usize),
                _ => None,
            }
        }
    }

    pub fn get_i8(&self) -> Option<i8> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::I128, data_union: DataUnion { i } } => Some(*i as i8),
                _ => None,
            }
        }
    }

    pub fn get_i16(&self) -> Option<i16> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::I128, data_union: DataUnion { i } } => Some(*i as i16),
                _ => None,
            }
        }
    }

    pub fn get_i32(&self) -> Option<i32> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::I128, data_union: DataUnion { i } } => Some(*i as i32),
                _ => None,
            }
        }
    }

    pub fn get_i64(&self) -> Option<i64> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::I128, data_union: DataUnion { i } } => Some(*i as i64),
                _ => None,
            }
        }
    }

    pub fn get_i128(&self) -> Option<i128> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::I128, data_union: DataUnion { i } } => Some(*i as i128),
                _ => None,
            }
        }
    }

    pub fn get_isize(&self) -> Option<isize> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::I128, data_union: DataUnion { i } } => Some(*i as isize),
                _ => None,
            }
        }
    }

    pub fn get_f32(&self) -> Option<f32> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::Float, data_union: DataUnion { f } } => Some((*f).into_inner() as f32),
                _ => None,
            }
        }
    }

    pub fn get_f64(&self) -> Option<f64> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::Float, data_union: DataUnion { f } } => Some((*f).into_inner() as f64),
                _ => None,
            }
        }
    }

    pub fn get_string(&self) -> Option<String> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::String, data_union: DataUnion { s } } => Some(s.deref().clone()),
                _ => None,
            }
        }
    }
}

impl std::clone::Clone for InstructionData {
    fn clone(&self) -> Self {
        unsafe {
            match self {
                InstructionData { data_type: DataType::Bool, data_union: DataUnion { b } } => InstructionData::from_bool(*b),
                InstructionData { data_type: DataType::U128, data_union: DataUnion { u } } => InstructionData::from_u128(*u),
                InstructionData { data_type: DataType::I128, data_union: DataUnion { i } } => InstructionData::from_i128(*i),
                InstructionData { data_type: DataType::Float, data_union: DataUnion { f } } => InstructionData::from_f64((*f).into_inner()),
                InstructionData { data_type: DataType::String, data_union: DataUnion { s } } => InstructionData::from_string(s.deref()),
            }
        }
    }
}

impl std::fmt::Debug for InstructionData {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        unsafe {
            match self {
                InstructionData { data_type: DataType::Bool, data_union: DataUnion { b } } => formatter
                    .debug_struct("InstructionData")
                    .field("data_type", &self.data_type)
                    .field("data_union.b", &b)
                    .finish(),
                InstructionData { data_type: DataType::U128, data_union: DataUnion { u } } => formatter
                    .debug_struct("InstructionData")
                    .field("data_type", &self.data_type)
                    .field("data_union.u", &u)
                    .finish(),
                InstructionData { data_type: DataType::I128, data_union: DataUnion { i } } => formatter
                    .debug_struct("InstructionData")
                    .field("data_type", &self.data_type)
                    .field("data_union.i", &i)
                    .finish(),
                InstructionData { data_type: DataType::Float, data_union: DataUnion { f } } => formatter
                    .debug_struct("InstructionData")
                    .field("data_type", &self.data_type)
                    .field("data_union.f", &f)
                    .finish(),
                InstructionData { data_type: DataType::String, data_union: DataUnion { s } } => formatter
                    .debug_struct("InstructionData")
                    .field("data_type", &self.data_type)
                    .field("data_union.s", s.deref())
                    .finish(),
            }
        }
    }
}

impl Drop for InstructionData {
    /// If we are holding a string, make sure it gets dropped. All the other values of the union are Copy and do not
    /// need any special handling
    fn drop(&mut self) {
        unsafe {
            match self {
                InstructionData { data_type: DataType::String, data_union: DataUnion { s } } => {
                    ManuallyDrop::drop(s);
                }
                _ => {
                    // No drop step needed for the Copy primitives
                }
            }
        }
    }
}

impl std::hash::Hash for InstructionData {
    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
        unsafe {
            match self.data_type {
                DataType::Bool => self.data_union.b.hash(state),
                DataType::U128 => self.data_union.u.hash(state),
                DataType::I128 => self.data_union.i.hash(state),
                DataType::Float => self.data_union.f.hash(state),
                DataType::String => self.data_union.s.hash(state),
            }
        }
    }
}

impl Eq for InstructionData {}
impl PartialEq for InstructionData {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            match (self.data_type, other.data_type) {
                (DataType::Bool, DataType::Bool) => self.data_union.b == other.data_union.b,
                (DataType::U128, DataType::U128) => self.data_union.u == other.data_union.u,
                (DataType::I128, DataType::I128) => self.data_union.i == other.data_union.i,
                (DataType::Float, DataType::Float) => self.data_union.f == other.data_union.f,
                (DataType::String, DataType::String) => self.data_union.s == other.data_union.s,
                _ => false,
            }
        }
    }
}

// TODO: macro_impl! for
// pub trait From<T> {
//     fn from(T) -> Self;
// }
// where T is [bool, u8, u16 ...]
// string probably needs to be unique
