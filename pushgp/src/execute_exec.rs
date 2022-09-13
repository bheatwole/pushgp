use crate::*;
use pushgp_macros::*;

pub type Exec = Code;

pub trait VirtualMachineMustHaveExec<Vm: 'static> {
    fn exec(&mut self) -> &mut Stack<Exec>;
}

/// Defines the name on top of the NAME stack as an instruction that will push the top item of the EXEC stack back
/// onto the EXEC stack.
#[stack_instruction(Exec)]
fn define(vm: &mut Vm, code: Exec, name: Name) {
    vm.engine_mut().define_name(name, code);
}

/// An iteration instruction that performs a loop (the body of which is taken from the EXEC stack) the number of
/// times indicated by the INTEGER argument, pushing an index (which runs from zero to one less than the number of
/// iterations) onto the INTEGER stack prior to each execution of the loop body. This is similar to CODE.DO*COUNT
/// except that it takes its code argument from the EXEC stack. This should be implemented as a macro that expands
/// into a call to EXEC.DO*RANGE. EXEC.DO*COUNT takes a single INTEGER argument (the number of times that the loop
/// will be executed) and a single EXEC argument (the body of the loop). If the provided INTEGER argument is
/// negative or zero then this becomes a NOOP. Otherwise it expands into:
///   ( 0 <1 - IntegerArg> EXEC.DO*RANGE <ExecArg> )
#[stack_instruction(Exec)]
fn do_n_count(vm: &mut Vm, code: Exec, count: Integer) {
    // NOOP if count <= 0
    if count <= 0 {
        // Put the items we popped back to make a NOOP
        vm.exec().push(code);
        vm.integer().push(count);
    } else {
        // Turn into DoNRange with (Count - 1) as destination
        let next = Code::new_list(vec![
            IntegerLiteralValue::new_code(vm, 0),
            IntegerLiteralValue::new_code(vm, count - 1),
            ExecDoNRange::new_code(vm),
            code,
        ]);
        vm.exec().push(next);
    }
}

/// An iteration instruction that executes the top item on the EXEC stack a number of times that depends on the top
/// two integers, while also pushing the loop counter onto the INTEGER stack for possible access during the
/// execution of the body of the loop. This is similar to CODE.DO*COUNT except that it takes its code argument from
/// the EXEC stack. The top integer is the "destination index" and the second integer is the "current index."
/// First the code and the integer arguments are saved locally and popped. Then the integers are compared. If the
/// integers are equal then the current index is pushed onto the INTEGER stack and the code (which is the "body" of
/// the loop) is pushed onto the EXEC stack for subsequent execution. If the integers are not equal then the current
/// index will still be pushed onto the INTEGER stack but two items will be pushed onto the EXEC stack -- first a
/// recursive call to EXEC.DO*RANGE (with the same code and destination index, but with a current index that has
/// been either incremented or decremented by 1 to be closer to the destination index) and then the body code. Note
/// that the range is inclusive of both endpoints; a call with integer arguments 3 and 5 will cause its body to be
/// executed 3 times, with the loop counter having the values 3, 4, and 5. Note also that one can specify a loop
/// that "counts down" by providing a destination index that is less than the specified current index.
#[stack_instruction(Exec)]
fn do_n_range(vm: &mut Vm, code: Exec, dest: Integer, cur: Integer) {
    // If we haven't reached the destination yet, push the next iteration onto the stack first.
    if cur != dest {
        let increment = if cur < dest { 1 } else { -1 };
        let next = Code::new_list(vec![
            IntegerLiteralValue::new_code(vm, cur + increment),
            IntegerLiteralValue::new_code(vm, dest),
            ExecDoNRange::new_code(vm),
            code.clone(),
        ]);
        vm.exec().push(next);
    }

    // Push the current index onto the int stack so its accessible in the loop
    vm.integer().push(cur);

    // Push the code to run onto the exec stack
    vm.exec().push(code);
}

/// Like EXEC.DO*COUNT but does not push the loop counter. This should be implemented as a macro that expands into
/// EXEC.DO*RANGE, similarly to the implementation of EXEC.DO*COUNT, except that a call to INTEGER.POP should be
/// tacked on to the front of the loop body code in the call to EXEC.DO*RANGE. This call to INTEGER.POP will remove
/// the loop counter, which will have been pushed by EXEC.DO*RANGE, prior to the execution of the loop body.
#[stack_instruction(Exec)]
fn do_n_times(vm: &mut Vm, code: Exec, count: Integer) {
    // NOOP if count <= 0
    if count <= 0 {
        vm.exec().push(code);
        vm.integer().push(count);
    } else {
        // The difference between Count and Times is that the 'current index' is not available to
        // the loop body. Pop that value first
        let code = Code::new_list(vec![IntegerPop::new_code(vm), code]);

        // Turn into DoNRange with (Count - 1) as destination
        let next = Code::new_list(vec![
            IntegerLiteralValue::new_code(vm, 0),
            IntegerLiteralValue::new_code(vm, count - 1),
            ExecDoNRange::new_code(vm),
            code,
        ]);
        vm.exec().push(next);
    }
}

/// Duplicates the top item on the EXEC stack. Does not pop its argument (which, if it did, would negate the effect
/// of the duplication!). This may be thought of as a "DO TWICE" instruction.
#[stack_instruction(Exec)]
fn dup(vm: &mut Vm) {
    vm.exec().duplicate_top_item();
}

/// Pushes TRUE if the top two items on the EXEC stack are equal, or FALSE otherwise.
#[stack_instruction(Exec)]
fn equal(vm: &mut Vm, a: Exec, b: Exec) {
    vm.bool().push(a == b);
}

/// Empties the EXEC stack. This may be thought of as a "HALT" instruction.
#[stack_instruction(Exec)]
fn flush(vm: &mut Vm) {
    vm.exec().clear();
}

/// If the top item of the BOOLEAN stack is TRUE then this removes the second item on the EXEC stack, leaving the
/// first item to be executed. If it is false then it removes the first item, leaving the second to be executed.
/// This is similar to CODE.IF except that it operates on the EXEC stack. This acts as a NOOP unless there are at
/// least two items on the EXEC stack and one item on the BOOLEAN stack.
#[stack_instruction(Exec)]
fn _if(vm: &mut Vm, true_branch: Exec, false_branch: Exec, switch_on: Bool) {
    vm.exec().push(if switch_on { true_branch } else { false_branch });
}

/// The Push implementation of the "K combinator". Removes the second item on the EXEC stack.
#[stack_instruction(Exec)]
fn k(vm: &mut Vm, keep: Exec, _discard: Exec) {
    vm.exec().push(keep);
}

/// Pops the EXEC stack. This may be thought of as a "DONT" instruction.
#[stack_instruction(Exec)]
fn pop(vm: &mut Vm, _popped: Exec) {}

/// Rotates the top three items on the EXEC stack, pulling the third item out and pushing it on top. This is
/// equivalent to "2 EXEC.YANK".
#[stack_instruction(Exec)]
fn rot(vm: &mut Vm) {
    vm.exec().rotate();
}

/// Inserts the top EXEC item "deep" in the stack, at the position indexed by the top INTEGER. This may be thought
/// of as a "DO LATER" instruction.
#[stack_instruction(Exec)]
fn shove(vm: &mut Vm, position: Integer) {
    if !vm.exec().shove(position) {
        vm.integer().push(position);
    }
}

/// Pushes the stack depth onto the INTEGER stack.
#[stack_instruction(Exec)]
fn stack_depth(vm: &mut Vm) {
    let len = vm.exec().len() as i64;
    vm.integer().push(len);
}

/// Swaps the top two items on the EXEC stack.
#[stack_instruction(Exec)]
fn swap(vm: &mut Vm) {
    vm.exec().swap();
}

/// The Push implementation of the "S combinator". Pops 3 items from the EXEC stack, which we will call A, B, and C
/// (with A being the first one popped). Then pushes a list containing B and C back onto the EXEC stack, followed by
/// another instance of C, followed by another instance of A.
#[stack_instruction(Exec)]
fn s(vm: &mut Vm, a: Exec, b: Exec, c: Exec) {
    vm.exec().push(Code::new_list(vec![b, c.clone()]));
    vm.exec().push(c);
    vm.exec().push(a);
}

/// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
/// The index is taken from the INTEGER stack.
#[stack_instruction(Exec)]
fn yank_dup(vm: &mut Vm, position: Integer) {
    if !vm.exec().yank_duplicate(position) {
        vm.integer().push(position);
    }
}

/// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the
/// INTEGER stack. This may be thought of as a "DO SOONER" instruction.
#[stack_instruction(Exec)]
fn yank(vm: &mut Vm, position: Integer) {
    if !vm.exec().yank(position) {
        vm.integer().push(position);
    }
}

/// The Push implementation of the "Y combinator". Inserts beneath the top item of the EXEC stack a new item of the
/// form "( EXEC.Y <TopItem> )".
#[stack_instruction(Exec)]
fn y(vm: &mut Vm, repeat: Exec) {
    // Construct the looping code
    let next_exec = Code::new_list(vec![ExecY::new_code(vm), repeat.clone()]);
    // Push them back so that we DO and the DO AGAIN
    vm.exec().push(next_exec);
    vm.exec().push(repeat);
}
