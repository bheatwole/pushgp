use crate::util::stack_to_vec;
use crate::InstructionData;
use std::cell::RefCell;

/// Defines a stack of values that uses interior mutability for all operations
pub trait StackTrait<T: Clone> {
    /// Returns the top item from the Stack or None if the stack is empty
    fn pop(&self) -> Option<T>;

    /// Pushes the specified item onto the top of the stack
    fn push(&self, item: T);

    /// Returns a clone of the top item from the Stack or None if the stack is empty
    fn peek(&self) -> Option<T>;

    /// Returns the length of the Stack
    fn len(&self) -> usize;

    /// Duplicates the top item of the stack. This should not change the Stack or panic if the stack is empty
    fn duplicate_top_item(&self);

    /// Deletes all items from the Stack
    fn clear(&self);

    /// Rotates the top three items on the stack, pulling the third item out and pushing it on top. This should not
    /// modify the stack if there are fewer than three items
    fn rotate(&self);

    /// Pops the top item of the stack and pushes it down the specified number of positions. Thus `shove(0)` has no
    /// effect. The position is taken modulus the original size of the stack. I.E. `shove(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `shove(2)` or `[ 'A', 'C', 'B' ]`.
    ///
    /// Returns true if a shove was performed (even if it had no effect)
    fn shove(&self, position: i64) -> bool;

    /// Reverses the position of the top two items on the stack. No effect if there are not at least two items.
    fn swap(&self);

    /// Removes an item by its index from deep in the stack and pushes it onto the top. The position is taken modulus
    /// the original size of the stack. I.E. `yank(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `yank(2)` or `[ 'B', 'A', 'C' ]`.
    ///
    /// Returns true if a yank was performed (even if it had no effect)
    fn yank(&self, position: i64) -> bool;

    /// Copies an item by its index from deep in the stack and pushes it onto the top. The position is taken modulus
    /// the original size of the stack. I.E. `yank_duplicate(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `yank_duplicate(2)` or `[ 'C', 'B', 'A', 'C' ]`.
    ///
    /// Returns true if a yank was performed (even if it had no effect)
    fn yank_duplicate(&self, position: i64) -> bool;
}

#[derive(Clone, Debug, PartialEq)]
pub struct InstructionDataStack {
    stack: RefCell<Vec<InstructionData>>,
}

impl InstructionDataStack {
    pub fn new() -> InstructionDataStack {
        InstructionDataStack { stack: RefCell::new(vec![]) }
    }

    pub fn new_from_vec(stack: Vec<InstructionData>) -> InstructionDataStack {
        InstructionDataStack { stack: RefCell::new(stack) }
    }
}

impl StackTrait<InstructionData> for InstructionDataStack {
    /// Returns the top item from the Stack or None if the stack is empty
    fn pop(&self) -> Option<InstructionData> {
        self.stack.borrow_mut().pop()
    }

    /// Pushes the specified item onto the top of the stack
    fn push(&self, item: InstructionData) {
        self.stack.borrow_mut().push(item)
    }

    /// Returns a clone of the top item from the Stack or None if the stack is empty
    fn peek(&self) -> Option<InstructionData> {
        self.stack.borrow_mut().last().map(|item| item.clone())
    }

    /// Returns the length of the Stack
    fn len(&self) -> usize {
        self.stack.borrow().len()
    }

    /// Duplicates the top item of the stack. This should not change the Stack or panic if the stack is empty
    fn duplicate_top_item(&self) {
        let mut duplicate = None;

        // This patten avoid mutable and immutable borrow of stack at the same time
        if let Some(top_item) = self.stack.borrow().last() {
            duplicate = Some(top_item.clone());
        }
        if let Some(new_item) = duplicate {
            self.stack.borrow_mut().push(new_item);
        }
    }

    /// Deletes all items from the Stack
    fn clear(&self) {
        self.stack.borrow_mut().clear()
    }

    /// Rotates the top three items on the stack, pulling the third item out and pushing it on top. This should not
    /// modify the stack if there are fewer than three items
    fn rotate(&self) {
        if self.stack.borrow().len() >= 3 {
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
    fn shove(&self, position: i64) -> bool {
        if self.stack.borrow().len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.borrow().len());
            let item = self.stack.borrow_mut().pop().unwrap();
            self.stack.borrow_mut().insert(vec_index, item);
            true
        } else {
            false
        }
    }

    /// Reverses the position of the top two items on the stack. No effect if there are not at least two items.
    fn swap(&self) {
        if self.stack.borrow().len() >= 2 {
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
    fn yank(&self, position: i64) -> bool {
        if self.stack.borrow().len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.borrow().len());
            let item = self.stack.borrow_mut().remove(vec_index);
            self.stack.borrow_mut().push(item);
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
    fn yank_duplicate(&self, position: i64) -> bool {
        if self.stack.borrow().len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.borrow().len());
            let duplicate = self.stack.borrow().get(vec_index).unwrap().clone();
            self.stack.borrow_mut().push(duplicate);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Stack<'a, T: Clone + From<InstructionData> + Into<InstructionData>> {
    stack: Option<&'a InstructionDataStack>,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T: Clone + From<InstructionData> + Into<InstructionData>> Stack<'a, T> {
    pub fn new(stack: Option<&'a InstructionDataStack>) -> Stack<T> {
        Stack::<'a, T> { stack, phantom: std::marker::PhantomData }
    }
}

impl<'a, T: Clone + From<InstructionData> + Into<InstructionData>> StackTrait<T> for Stack<'a, T> {
    /// Returns the top item from the Stack or None if the stack is empty
    fn pop(&self) -> Option<T> {
        if let Some(stack) = self.stack {
            stack.pop().map(|v| std::convert::From::from(v))
        } else {
            None
        }
    }

    /// Pushes the specified item onto the top of the stack
    fn push(&self, item: T) {
        if let Some(stack) = self.stack {
            stack.push(item.into())
        }
    }

    /// Returns a clone of the top item from the Stack or None if the stack is empty
    fn peek(&self) -> Option<T> {
        if let Some(stack) = self.stack {
            stack.peek().map(|v| std::convert::From::from(v))
        } else {
            None
        }
    }

    /// Returns the length of the Stack
    fn len(&self) -> usize {
        if let Some(stack) = self.stack {
            stack.len()
        } else {
            0
        }
    }

    /// Duplicates the top item of the stack. This should not change the Stack or panic if the stack is empty
    fn duplicate_top_item(&self) {
        if let Some(stack) = self.stack {
            stack.duplicate_top_item()
        }
    }

    /// Deletes all items from the Stack
    fn clear(&self) {
        if let Some(stack) = self.stack {
            stack.clear()
        }
    }

    /// Rotates the top three items on the stack, pulling the third item out and pushing it on top. This should not
    /// modify the stack if there are fewer than three items
    fn rotate(&self) {
        if let Some(stack) = self.stack {
            stack.rotate()
        }
    }

    /// Pops the top item of the stack and pushes it down the specified number of positions. Thus `shove(0)` has no
    /// effect. The position is taken modulus the original size of the stack. I.E. `shove(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `shove(2)` or `[ 'A', 'C', 'B' ]`.
    ///
    /// Returns true if a shove was performed (even if it had no effect)
    fn shove(&self, position: i64) -> bool {
        if let Some(stack) = self.stack {
            stack.shove(position)
        } else {
            false
        }
    }

    /// Reverses the position of the top two items on the stack. No effect if there are not at least two items.
    fn swap(&self) {
        if let Some(stack) = self.stack {
            stack.swap()
        }
    }

    /// Removes an item by its index from deep in the stack and pushes it onto the top. The position is taken modulus
    /// the original size of the stack. I.E. `yank(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `yank(2)` or `[ 'B', 'A', 'C' ]`.
    ///
    /// Returns true if a yank was performed (even if it had no effect)
    fn yank(&self, position: i64) -> bool {
        if let Some(stack) = self.stack {
            stack.yank(position)
        } else {
            false
        }
    }

    /// Copies an item by its index from deep in the stack and pushes it onto the top. The position is taken modulus
    /// the original size of the stack. I.E. `yank_duplicate(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `yank_duplicate(2)` or `[ 'C', 'B', 'A', 'C' ]`.
    ///
    /// Returns true if a yank was performed (even if it had no effect)
    fn yank_duplicate(&self, position: i64) -> bool {
        if let Some(stack) = self.stack {
            stack.yank_duplicate(position)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{InstructionData, InstructionDataStack, Stack, StackTrait};

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
