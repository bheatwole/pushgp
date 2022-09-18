use pushgp::StaticName;
use pushgp_macros::stack_instruction;

use crate::{VirtualMachineMustHaveInstructionName, VirtualMachineMustHaveWeight};


#[stack_instruction(Exec)]
fn set_instruction_weight(vm: &mut Vm, instruction_name: InstructionName, weight: Weight) {
    vm.engine_mut().get_configuration_mut().set_instruction_weight(instruction_name, weight);
}