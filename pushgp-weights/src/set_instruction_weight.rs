use pushgp::StaticName;
use pushgp_macros::stack_instruction;

use crate::{VirtualMachineMustHaveInstructionName, VirtualMachineMustHaveWeight};


#[stack_instruction(Exec)]
fn set_instruction_weight(vm: &mut Vm, instruction_name: InstructionName, weight: Weight) {
    let mut config = vm.engine().get_configuration().clone();
    config.set_instruction_weight(instruction_name, weight);
    vm.engine_mut().reset_configuration(config);
}