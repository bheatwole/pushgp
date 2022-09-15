use get_size::GetSize;

use crate::util::stack_to_vec;

#[derive(Debug, PartialEq)]
pub struct Stack<T: Clone + GetSize> {
    stack: Vec<T>,
    mem_size: usize,
}

impl<T: Clone + GetSize> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { stack: vec![], mem_size: 0 }
    }

    pub fn new_from_vec(stack: Vec<T>) -> Stack<T> {
        let mem_size = stack.iter().map(|item| item.get_size()).sum();
        Stack { stack, mem_size }
    }

    /// Returns the top item from the Stack or None if the stack is empty
    pub fn pop(&mut self) -> Option<T> {
        let item = self.stack.pop();
        if let Some(v) = item.as_ref() {
            self.mem_size -= v.get_size();
        }
        item
    }

    /// Returns a clone of the top item from the Stack or None if the stack is empty
    pub fn peek(&self) -> Option<T> {
        self.stack.last().map(|item| item.clone())
    }

    /// Pushes the specified item onto the top of the stack
    pub fn push(&mut self, item: T) {
        self.mem_size += item.get_size();
        self.stack.push(item)
    }

    /// Returns the length of the Stack
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn size_of(&self) -> usize {
        self.mem_size
    }

    /// Duplicates the top item of the stack. This should not change the Stack or panic if the stack is empty
    pub fn duplicate_top_item(&mut self) {
        let mut duplicate = None;

        // This patten avoid mutable and immutable borrow of stack at the same time
        if let Some(top_item) = self.stack.last() {
            duplicate = Some(top_item.clone());
        }
        if let Some(new_item) = duplicate {
            self.push(new_item);
        }
    }

    /// Deletes all items from the Stack
    pub fn clear(&mut self) {
        self.mem_size = 0;
        self.stack.clear()
    }

    /// Rotates the top three items on the stack, pulling the third item out and pushing it on top. This should not
    /// modify the stack if there are fewer than three items
    pub fn rotate(&mut self) {
        if self.stack.len() >= 3 {
            let first = self.pop().unwrap();
            let second = self.pop().unwrap();
            let third = self.pop().unwrap();
            self.push(second);
            self.push(first);
            self.push(third);
        }
    }

    /// Pops the top item of the stack and pushes it down the specified number of positions. Thus `shove(0)` has no
    /// effect. The position is taken modulus the original size of the stack. I.E. `shove(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `shove(2)` or `[ 'A', 'C', 'B' ]`.
    ///
    /// Returns true if a shove was performed (even if it had no effect)
    pub fn shove(&mut self, position: i64) -> bool {
        if self.stack.len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.len());
            let item = self.pop().unwrap();
            self.mem_size += item.get_size();
            self.stack.insert(vec_index, item);
            true
        } else {
            false
        }
    }

    /// Reverses the position of the top two items on the stack. No effect if there are not at least two items.
    pub fn swap(&mut self) {
        if self.stack.len() >= 2 {
            let first = self.pop().unwrap();
            let second = self.pop().unwrap();
            self.push(first);
            self.push(second);
        }
    }

    /// Removes an item by its index from deep in the stack and pushes it onto the top. The position is taken modulus
    /// the original size of the stack. I.E. `yank(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `yank(2)` or `[ 'B', 'A', 'C' ]`.
    ///
    /// Returns true if a yank was performed (even if it had no effect)
    pub fn yank(&mut self, position: i64) -> bool {
        if self.stack.len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.len());
            let item = self.stack.remove(vec_index);
            self.mem_size -= item.get_size();
            self.push(item);
            true
        } else {
            false
        }
    }

    /// Copies an item by its index from deep in the stack and pushes it onto the top. The position is taken modulus
    /// the original size of the stack. I.E. `yank_duplicate(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `yank_duplicate(2)` or `[ 'C', 'B', 'A', 'C' ]`.
    ///
    /// Returns true if a yank was performed (even if it had no effect)
    pub fn yank_duplicate(&mut self, position: i64) -> bool {
        if self.stack.len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.len());
            let duplicate = self.stack.get(vec_index).unwrap().clone();
            self.push(duplicate);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {

    use get_size::GetSize;

    use crate::*;

    fn new_base_vm() -> BaseVm {
        let mut vm = BaseVm::new(None, Configuration::new_simple());
        add_base_instructions(&mut vm);
        add_base_literals(&mut vm);

        vm
    }

    #[test]
    fn stack_bool_mem_size() {
        assert_eq!(1, std::mem::size_of::<bool>());
        let mut stack = Stack::new();
        stack.push(true);
        stack.push(false);
        assert_eq!(2, stack.size_of());
    }

    #[test]
    fn stack_code_mem_size() {
        let mut vm = new_base_vm();

        // The size of Data is 32 bytes
        assert_eq!(32, std::mem::size_of::<Data>());
        // Code adds on 8 bytes
        assert_eq!(40, std::mem::size_of::<Code>());

        // Size (stack + heap) of a simple instruction is zero
        assert_eq!(0, std::mem::size_of::<IntegerSum>());

        // // Size (stack + heap) of a value-holding instruction is also zero because the value is held in Data
        assert_eq!(0, std::mem::size_of::<BoolLiteralValue>());
        assert_eq!(0, std::mem::size_of::<FloatLiteralValue>());
        assert_eq!(0, std::mem::size_of::<IntegerLiteralValue>());
        assert_eq!(0, std::mem::size_of::<NameLiteralValue>());

        // The size of a stack of Code should be the 40 times the number of elements
        let mut stack = Stack::<Code>::new();
        stack.push(vm.random_value::<IntegerSum>());
        stack.push(vm.random_value::<BoolLiteralValue>());
        stack.push(vm.random_value::<FloatLiteralValue>());
        stack.push(vm.random_value::<IntegerLiteralValue>());
        stack.push(vm.random_value::<NameLiteralValue>());
        let mut expected_size = stack.len() * 40;
        assert_eq!(expected_size, stack.size_of());

        // Finally, the size of a list of those same instructions should be the same as the stack + the size of one more
        // Code
        let list = Code::new_list(vec![
            vm.random_value::<IntegerSum>(),
            vm.random_value::<BoolLiteralValue>(),
            vm.random_value::<FloatLiteralValue>(),
            vm.random_value::<IntegerLiteralValue>(),
            vm.random_value::<NameLiteralValue>(),
        ]);
        expected_size += std::mem::size_of::<Code>();
        assert_eq!(expected_size, list.get_size());
    }

    #[test]
    fn stack_push_pop() {
        // Start with an empty stack
        let mut stack = Stack::new();
        assert_eq!(0, stack.len());

        // Push an item and ensure it got there
        stack.push(1);
        assert_eq!(1, stack.len());

        // Pop the item and confirm the stack is empty
        assert_eq!(Some(1), stack.pop());
        assert_eq!(0, stack.len());

        // Popping empty stack is None
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn stack_duplicate_top_item() {
        let mut stack = Stack::new();
        assert_eq!(0, stack.len());

        // Duplicating an empty stack has no effect
        stack.duplicate_top_item();
        assert_eq!(0, stack.len());
        assert_eq!(None, stack.pop());

        // Push and duplicate an item
        stack.push(1);
        stack.duplicate_top_item();
        assert_eq!(2, stack.len());

        // Confirm the stack contents
        assert_eq!(Some(1), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn stack_clear() {
        let mut stack = Stack::new();
        assert_eq!(0, stack.len());

        // Add some items
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(3, stack.len());

        // Clear the stack and check that it is empty
        stack.clear();
        assert_eq!(0, stack.len());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn stack_rotate() {
        let mut stack = Stack::new();
        assert_eq!(0, stack.len());

        // Add two items
        stack.push(1);
        stack.push(2);
        assert_eq!(2, stack.len());

        // Rotate requires three items and so should have no effect here
        stack.rotate();
        assert_eq!(Some(2), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());

        // Add three items this time
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(3, stack.len());

        // Rotate will pull that third item up to the top
        stack.rotate();
        assert_eq!(Some(1), stack.pop());
        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(2), stack.pop());
        assert_eq!(None, stack.pop());

        // Add four items this time
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        assert_eq!(4, stack.len());

        // Rotate will pull that third item up to the top
        stack.rotate();
        assert_eq!(Some(2), stack.pop());
        assert_eq!(Some(4), stack.pop());
        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn stack_shove() {
        let mut stack = Stack::<char>::new();
        assert_eq!(0, stack.len());

        // Shoving an empty stack returns zero
        assert_eq!(false, stack.shove(1));
        assert_eq!(false, stack.shove(500));
        assert_eq!(false, stack.shove(-1));

        // Shoving a single value succeeds but has no effect
        let mut stack = Stack::new_from_vec(vec!['A']);
        let expected = Stack::new_from_vec(vec!['A']);
        assert_eq!(true, stack.shove(1));
        assert_eq!(expected, stack);
        assert_eq!(true, stack.shove(500));
        assert_eq!(expected, stack);
        assert_eq!(true, stack.shove(-1));
        assert_eq!(expected, stack);

        // Perform some shoves
        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(0));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(1));
        let expected = Stack::new_from_vec(vec!['C', 'A', 'B']);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(2));
        let expected = Stack::new_from_vec(vec!['A', 'C', 'B']);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(3));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(expected, stack);

        // Negative works too
        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(-1));
        let expected = Stack::new_from_vec(vec!['A', 'C', 'B']);
        assert_eq!(expected, stack);
    }

    #[test]
    fn stack_swap() {
        let mut stack = Stack::new();
        assert_eq!(0, stack.len());

        // Add an item
        stack.push(1);
        assert_eq!(1, stack.len());

        // Swap requires two items and so should have no effect here
        stack.swap();
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());

        // Add two items this time
        stack.push(1);
        stack.push(2);
        assert_eq!(2, stack.len());

        // Swap will exchange the top two items
        stack.swap();
        assert_eq!(Some(1), stack.pop());
        assert_eq!(Some(2), stack.pop());
        assert_eq!(None, stack.pop());

        // Add three items this time
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(3, stack.len());

        // Swap will exchange just the top two items
        stack.swap();
        assert_eq!(Some(2), stack.pop());
        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn stack_yank() {
        let mut stack = Stack::<i64>::new();
        assert_eq!(0, stack.len());

        // Yanking an empty stack returns false
        assert_eq!(false, stack.yank(1));
        assert_eq!(false, stack.yank(500));
        assert_eq!(false, stack.yank(-1));

        // Yanking a single value succeeds but has no effect
        let mut stack = Stack::new_from_vec(vec!['A']);
        let expected = Stack::new_from_vec(vec!['A']);
        assert_eq!(true, stack.yank(1));
        assert_eq!(expected, stack);
        assert_eq!(true, stack.yank(500));
        assert_eq!(expected, stack);
        assert_eq!(true, stack.yank(-1));
        assert_eq!(expected, stack);

        // Perform some yanks
        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(0));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(1));
        let expected = Stack::new_from_vec(vec!['C', 'A', 'B']);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(2));
        let expected = Stack::new_from_vec(vec!['B', 'A', 'C']);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(3));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(expected, stack);

        // Negative works too
        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(-1));
        let expected = Stack::new_from_vec(vec!['B', 'A', 'C']);
        assert_eq!(expected, stack);
    }

    #[test]
    fn stack_yank_duplicate() {
        let mut stack = Stack::<char>::new();
        assert_eq!(0, stack.len());

        // Yanking an empty stack returns false
        assert_eq!(false, stack.yank_duplicate(1));
        assert_eq!(false, stack.yank_duplicate(500));
        assert_eq!(false, stack.yank_duplicate(-1));

        // Yanking a single value is easy
        let mut stack = Stack::new_from_vec(vec!['A']);
        assert_eq!(true, stack.yank_duplicate(1));
        let expected = Stack::new_from_vec(vec!['A', 'A']);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['A']);
        assert_eq!(true, stack.yank_duplicate(500));
        let expected = Stack::new_from_vec(vec!['A', 'A']);
        assert_eq!(expected, stack);

        // Perform some more complicated yanks
        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(0));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A', 'A']);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(1));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A', 'B']);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(2));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A', 'C']);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(3));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A', 'A']);
        assert_eq!(expected, stack);

        // Negative works too
        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(-1));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A', 'C']);
        assert_eq!(expected, stack);
    }
}
