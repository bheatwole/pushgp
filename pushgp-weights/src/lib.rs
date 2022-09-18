extern crate pushgp_macros;

mod instruction_name;
mod set_instruction_weight;
mod vm;
mod weight;

pub use instruction_name::*;
pub use set_instruction_weight::*;
pub use vm::*;
pub use weight::*;