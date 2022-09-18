use pushgp::StaticName;
use pushgp_macros::stack_instruction;

use crate::{VirtualMachineMustHaveWeight, VirtualMachineMustHaveTarget};


#[stack_instruction(Exec)]
fn set_instruction_weight(vm: &mut Vm, instruction_name_index: Integer, weight: Weight) {
    let index_usize: usize = instruction_name_index.saturating_abs() as usize;
    vm.target().set_target_weight(index_usize, weight);
}