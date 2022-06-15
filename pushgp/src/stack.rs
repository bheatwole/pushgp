use crate::util::stack_to_vec;

pub struct Stack<T: Clone> {
    stack: Vec<T>,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { stack: vec![] }
    }

    pub fn new_from_vec(stack: Vec<T>) -> Stack<T> {
        Stack { stack }
    }

    /// Returns the top item from the Stack or None if the stack is empty
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    /// Pushes the specified item onto the top of the stack
    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    /// Returns the length of the Stack
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Duplicates the top item of the stack. This should not change the Stack or panic if the stack is empty
    pub fn duplicate_top_item(&mut self) {
        let mut duplicate = None;

        // This patten avoid mutable and immutable borrow of stack at the same time
        if let Some(top_item) = self.stack.last() {
            duplicate = Some(top_item.clone());
        }
        if let Some(new_item) = duplicate {
            self.stack.push(new_item);
        }
    }

    /// Deletes all items from the Stack
    pub fn clear(&mut self) {
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
            let item = self.stack.pop().unwrap();
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
            self.stack.push(item);
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
            self.stack.push(duplicate);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Stack;

    fn new_stack<'a, T: Clone + From<InstructionData> + Into<InstructionData>>(
        base_stack: &'a InstructionDataStack,
    ) -> Stack<'a, T> {
        base_stack.clear();
        Stack::new(Some(&base_stack))
    }

    fn new_from_vec<'a, T: Clone + From<InstructionData> + Into<InstructionData>>(
        base_stack: &'a InstructionDataStack,
        data: Vec<T>,
    ) -> Stack<'a, T> {
        base_stack.clear();
        for item in data.iter() {
            base_stack.push(item.clone().into());
        }
        Stack::new(Some(&base_stack))
    }

    impl From<InstructionData> for char {
        fn from(data: InstructionData) -> Self {
            data.get_u8().unwrap() as char
        }
    }
    impl Into<InstructionData> for char {
        fn into(self) -> InstructionData {
            InstructionData::from_u8(self as u8)
        }
    }

    #[test]
    fn stack_push_pop() {
        // Start with an empty stack
        let base_stack = InstructionDataStack::new();
        let stack = new_stack(&base_stack);
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
        let base_stack = InstructionDataStack::new();
        let stack = new_stack(&base_stack);
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
        let base_stack = InstructionDataStack::new();
        let stack = new_stack(&base_stack);
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
        let base_stack = InstructionDataStack::new();
        let stack = new_stack(&base_stack);
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
        let base_stack = InstructionDataStack::new();
        let expected_stack = InstructionDataStack::new();
        let stack = new_stack::<char>(&base_stack);
        assert_eq!(0, stack.len());

        // Shoving an empty stack returns zero
        assert_eq!(false, stack.shove(1));
        assert_eq!(false, stack.shove(500));
        assert_eq!(false, stack.shove(-1));

        // Shoving a single value succeeds but has no effect
        let stack = new_from_vec(&base_stack, vec!['A']);
        let expected = new_from_vec(&expected_stack, vec!['A']);
        assert_eq!(true, stack.shove(1));
        assert_eq!(expected, stack);
        assert_eq!(true, stack.shove(500));
        assert_eq!(expected, stack);
        assert_eq!(true, stack.shove(-1));
        assert_eq!(expected, stack);

        // Perform some shoves
        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(0));
        let expected = new_from_vec(&expected_stack, vec!['C', 'B', 'A']);
        assert_eq!(expected, stack);

        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(1));
        let expected = new_from_vec(&expected_stack, vec!['C', 'A', 'B']);
        assert_eq!(expected, stack);

        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(2));
        let expected = new_from_vec(&expected_stack, vec!['A', 'C', 'B']);
        assert_eq!(expected, stack);

        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(3));
        let expected = new_from_vec(&expected_stack, vec!['C', 'B', 'A']);
        assert_eq!(expected, stack);

        // Negative works too
        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(-1));
        let expected = new_from_vec(&expected_stack, vec!['A', 'C', 'B']);
        assert_eq!(expected, stack);
    }

    #[test]
    fn stack_swap() {
        let base_stack = InstructionDataStack::new();
        let stack = new_stack(&base_stack);
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
        let base_stack = InstructionDataStack::new();
        let expected_stack = InstructionDataStack::new();
        let stack = new_stack::<i64>(&base_stack);
        assert_eq!(0, stack.len());

        // Yanking an empty stack returns false
        assert_eq!(false, stack.yank(1));
        assert_eq!(false, stack.yank(500));
        assert_eq!(false, stack.yank(-1));

        // Yanking a single value succeeds but has no effect
        let stack = new_from_vec(&base_stack, vec!['A']);
        let expected = new_from_vec(&expected_stack, vec!['A']);
        assert_eq!(true, stack.yank(1));
        assert_eq!(expected, stack);
        assert_eq!(true, stack.yank(500));
        assert_eq!(expected, stack);
        assert_eq!(true, stack.yank(-1));
        assert_eq!(expected, stack);

        // Perform some yanks
        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(0));
        let expected = new_from_vec(&expected_stack, vec!['C', 'B', 'A']);
        assert_eq!(expected, stack);

        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(1));
        let expected = new_from_vec(&expected_stack, vec!['C', 'A', 'B']);
        assert_eq!(expected, stack);

        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(2));
        let expected = new_from_vec(&expected_stack, vec!['B', 'A', 'C']);
        assert_eq!(expected, stack);

        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(3));
        let expected = new_from_vec(&expected_stack, vec!['C', 'B', 'A']);
        assert_eq!(expected, stack);

        // Negative works too
        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(-1));
        let expected = new_from_vec(&expected_stack, vec!['B', 'A', 'C']);
        assert_eq!(expected, stack);
    }

    #[test]
    fn stack_yank_duplicate() {
        let base_stack = InstructionDataStack::new();
        let expected_stack = InstructionDataStack::new();
        let stack = new_stack::<char>(&base_stack);
        assert_eq!(0, stack.len());

        // Yanking an empty stack returns false
        assert_eq!(false, stack.yank_duplicate(1));
        assert_eq!(false, stack.yank_duplicate(500));
        assert_eq!(false, stack.yank_duplicate(-1));

        // Yanking a single value is easy
        let stack = new_from_vec(&base_stack, vec!['A']);
        assert_eq!(true, stack.yank_duplicate(1));
        let expected = new_from_vec(&expected_stack, vec!['A', 'A']);
        assert_eq!(expected, stack);

        let stack = new_from_vec(&base_stack, vec!['A']);
        assert_eq!(true, stack.yank_duplicate(500));
        let expected = new_from_vec(&expected_stack, vec!['A', 'A']);
        assert_eq!(expected, stack);

        // Perform some more complicated yanks
        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(0));
        let expected = new_from_vec(&expected_stack, vec!['C', 'B', 'A', 'A']);
        assert_eq!(expected, stack);

        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(1));
        let expected = new_from_vec(&expected_stack, vec!['C', 'B', 'A', 'B']);
        assert_eq!(expected, stack);

        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(2));
        let expected = new_from_vec(&expected_stack, vec!['C', 'B', 'A', 'C']);
        assert_eq!(expected, stack);

        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(3));
        let expected = new_from_vec(&expected_stack, vec!['C', 'B', 'A', 'A']);
        assert_eq!(expected, stack);

        // Negative works too
        let stack = new_from_vec(&base_stack, vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(-1));
        let expected = new_from_vec(&expected_stack, vec!['C', 'B', 'A', 'C']);
        assert_eq!(expected, stack);
    }
}
