use crate::{util::stack_to_vec, ExecutionError};

#[derive(Clone, Debug, PartialEq)]
pub struct Stack<T: Clone> {
    stack: Vec<T>,
    max_len: usize,
}

impl<T: Clone> Stack<T> {
    pub fn new(max_len: usize) -> Stack<T> {
        Stack { stack: vec![], max_len: max_len }
    }

    pub fn new_from_vec(stack: Vec<T>, max_len: usize) -> Stack<T> {
        Stack { stack, max_len }
    }

    /// Returns the top item from the Stack or None if the stack is empty
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    /// Returns a clone of the top item from the Stack or None if the stack is empty
    pub fn peek(&self) -> Option<T> {
        self.stack.last().map(|item| item.clone())
    }

    /// Pushes the specified item onto the top of the stack
    pub fn push(&mut self, item: T) -> Result<(), ExecutionError> {
        if self.stack.len() < self.max_len {
            self.stack.push(item);
            Ok(())
        } else {
            Err(ExecutionError::OutOfMemory)
        }
    }

    /// Returns the length of the Stack
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Duplicates the top item of the stack. This should not change the Stack or panic if the stack is empty
    pub fn duplicate_top_item(&mut self) -> Result<(), ExecutionError> {
        if self.stack.len() < self.max_len {
            let mut duplicate = None;

            // This patten avoids mutable and immutable borrow of stack at the same time
            if let Some(top_item) = self.stack.last() {
                duplicate = Some(top_item.clone());
            }
            if let Some(new_item) = duplicate {
                self.push(new_item)?;
            }
            Ok(())
        } else {
            Err(ExecutionError::OutOfMemory)
        }
    }

    /// Deletes all items from the Stack
    pub fn clear(&mut self) {
        self.stack.clear()
    }

    /// Rotates the top three items on the stack, pulling the third item out and pushing it on top. This should not
    /// modify the stack if there are fewer than three items
    pub fn rotate(&mut self) -> Result<(), ExecutionError> {
        if self.stack.len() >= 3 {
            let first = self.pop().unwrap();
            let second = self.pop().unwrap();
            let third = self.pop().unwrap();
            self.push(second)?;
            self.push(first)?;
            self.push(third)?;
            Ok(())
        } else {
            Err(ExecutionError::InsufficientInputs)
        }
    }

    /// Pops the top item of the stack and pushes it down the specified number of positions. Thus `shove(0)` has no
    /// effect. The position is taken modulus the original size of the stack. I.E. `shove(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `shove(2)` or `[ 'A', 'C', 'B' ]`.
    ///
    /// Returns Ok if a shove was performed (even if it had no effect)
    pub fn shove(&mut self, position: i64) -> Result<(), ExecutionError> {
        if self.stack.len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.len());
            let item = self.pop().unwrap();
            self.stack.insert(vec_index, item);
            Ok(())
        } else {
            Err(ExecutionError::InsufficientInputs)
        }
    }

    /// Reverses the position of the top two items on the stack. No effect if there are not at least two items.
    pub fn swap(&mut self) -> Result<(), ExecutionError> {
        if self.stack.len() >= 2 {
            let first = self.pop().unwrap();
            let second = self.pop().unwrap();
            self.push(first)?;
            self.push(second)?;
            Ok(())
        } else {
            Err(ExecutionError::InsufficientInputs)
        }
    }

    /// Removes an item by its index from deep in the stack and pushes it onto the top. The position is taken modulus
    /// the original size of the stack. I.E. `yank(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `yank(2)` or `[ 'B', 'A', 'C' ]`.
    ///
    /// Returns Ok if a yank was performed (even if it had no effect)
    pub fn yank(&mut self, position: i64) -> Result<(), ExecutionError> {
        if self.stack.len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.len());
            let item = self.stack.remove(vec_index);
            self.push(item)?;
            Ok(())
        } else {
            Err(ExecutionError::InsufficientInputs)
        }
    }

    /// Copies an item by its index from deep in the stack and pushes it onto the top. The position is taken modulus
    /// the original size of the stack. I.E. `yank_duplicate(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `yank_duplicate(2)` or `[ 'C', 'B', 'A', 'C' ]`.
    ///
    /// Returns Ok if a yank was performed (even if it had no effect)
    pub fn yank_duplicate(&mut self, position: i64) -> Result<(), ExecutionError> {
        if self.stack.len() < self.max_len && self.stack.len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.len());
            let duplicate = self.stack.get(vec_index).unwrap().clone();
            self.push(duplicate)?;
            Ok(())
        } else {
            Err(ExecutionError::InsufficientInputs)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn stack_push_pop() {
        // Start with an empty stack
        let mut stack = Stack::new(5);
        assert_eq!(0, stack.len());

        // Push an item and ensure it got there
        assert_eq!(Ok(()), stack.push(1));
        assert_eq!(1, stack.len());

        // Pop the item and confirm the stack is empty
        assert_eq!(Some(1), stack.pop());
        assert_eq!(0, stack.len());

        // Popping empty stack is None
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn stack_duplicate_top_item() {
        let mut stack = Stack::new(5);
        assert_eq!(0, stack.len());

        // Duplicating an empty stack has no effect
        assert_eq!(Ok(()), stack.duplicate_top_item());
        assert_eq!(0, stack.len());
        assert_eq!(None, stack.pop());

        // Push and duplicate an item
        assert_eq!(Ok(()), stack.push(1));
        assert_eq!(Ok(()), stack.duplicate_top_item());
        assert_eq!(2, stack.len());

        // Confirm the stack contents
        assert_eq!(Some(1), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn stack_clear() {
        let mut stack = Stack::new(5);
        assert_eq!(0, stack.len());

        // Add some items
        assert_eq!(Ok(()), stack.push(1));
        assert_eq!(Ok(()), stack.push(2));
        assert_eq!(Ok(()), stack.push(3));
        assert_eq!(3, stack.len());

        // Clear the stack and check that it is empty
        stack.clear();
        assert_eq!(0, stack.len());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn stack_rotate() {
        let mut stack = Stack::new(5);
        assert_eq!(0, stack.len());

        // Add two items
        assert_eq!(Ok(()), stack.push(1));
        assert_eq!(Ok(()), stack.push(2));
        assert_eq!(2, stack.len());

        // Rotate requires three items and so should have no effect here
        assert_eq!(Err(ExecutionError::InsufficientInputs), stack.rotate());
        assert_eq!(Some(2), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());

        // Add three items this time
        assert_eq!(Ok(()), stack.push(1));
        assert_eq!(Ok(()), stack.push(2));
        assert_eq!(Ok(()), stack.push(3));
        assert_eq!(3, stack.len());

        // Rotate will pull that third item up to the top
        assert_eq!(Ok(()), stack.rotate());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(2), stack.pop());
        assert_eq!(None, stack.pop());

        // Add four items this time
        assert_eq!(Ok(()), stack.push(1));
        assert_eq!(Ok(()), stack.push(2));
        assert_eq!(Ok(()), stack.push(3));
        assert_eq!(Ok(()), stack.push(4));
        assert_eq!(4, stack.len());

        // Rotate will pull that third item up to the top
        assert_eq!(Ok(()), stack.rotate());
        assert_eq!(Some(2), stack.pop());
        assert_eq!(Some(4), stack.pop());
        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn stack_shove() {
        let mut stack = Stack::<char>::new(5);
        assert_eq!(0, stack.len());

        // Shoving an empty stack returns zero
        assert_eq!(Err(ExecutionError::InsufficientInputs), stack.shove(1));
        assert_eq!(Err(ExecutionError::InsufficientInputs), stack.shove(500));
        assert_eq!(Err(ExecutionError::InsufficientInputs), stack.shove(-1));

        // Shoving a single value succeeds but has no effect
        let mut stack = Stack::new_from_vec(vec!['A'], 5);
        let expected = Stack::new_from_vec(vec!['A'], 5);
        assert_eq!(Ok(()), stack.shove(1));
        assert_eq!(expected, stack);
        assert_eq!(Ok(()), stack.shove(500));
        assert_eq!(expected, stack);
        assert_eq!(Ok(()), stack.shove(-1));
        assert_eq!(expected, stack);

        // Perform some shoves
        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.shove(0));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.shove(1));
        let expected = Stack::new_from_vec(vec!['C', 'A', 'B'], 5);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.shove(2));
        let expected = Stack::new_from_vec(vec!['A', 'C', 'B'], 5);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.shove(3));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(expected, stack);

        // Negative works too
        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.shove(-1));
        let expected = Stack::new_from_vec(vec!['A', 'C', 'B'], 5);
        assert_eq!(expected, stack);
    }

    #[test]
    fn stack_swap() {
        let mut stack = Stack::new(5);
        assert_eq!(0, stack.len());

        // Add an item
        assert_eq!(Ok(()), stack.push(1));
        assert_eq!(1, stack.len());

        // Swap requires two items and so should have no effect here
        assert_eq!(Err(ExecutionError::InsufficientInputs), stack.swap());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());

        // Add two items this time
        assert_eq!(Ok(()), stack.push(1));
        assert_eq!(Ok(()), stack.push(2));
        assert_eq!(2, stack.len());

        // Swap will exchange the top two items
        assert_eq!(Ok(()), stack.swap());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(Some(2), stack.pop());
        assert_eq!(None, stack.pop());

        // Add three items this time
        assert_eq!(Ok(()), stack.push(1));
        assert_eq!(Ok(()), stack.push(2));
        assert_eq!(Ok(()), stack.push(3));
        assert_eq!(3, stack.len());

        // Swap will exchange just the top two items
        assert_eq!(Ok(()), stack.swap());
        assert_eq!(Some(2), stack.pop());
        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn stack_yank() {
        let mut stack = Stack::<i64>::new(5);
        assert_eq!(0, stack.len());

        // Yanking an empty stack returns false
        assert_eq!(Err(ExecutionError::InsufficientInputs), stack.yank(1));
        assert_eq!(Err(ExecutionError::InsufficientInputs), stack.yank(500));
        assert_eq!(Err(ExecutionError::InsufficientInputs), stack.yank(-1));

        // Yanking a single value succeeds but has no effect
        let mut stack = Stack::new_from_vec(vec!['A'], 5);
        let expected = Stack::new_from_vec(vec!['A'], 5);
        assert_eq!(Ok(()), stack.yank(1));
        assert_eq!(expected, stack);
        assert_eq!(Ok(()), stack.yank(500));
        assert_eq!(expected, stack);
        assert_eq!(Ok(()), stack.yank(-1));
        assert_eq!(expected, stack);

        // Perform some yanks
        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.yank(0));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.yank(1));
        let expected = Stack::new_from_vec(vec!['C', 'A', 'B'], 5);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.yank(2));
        let expected = Stack::new_from_vec(vec!['B', 'A', 'C'], 5);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.yank(3));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(expected, stack);

        // Negative works too
        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.yank(-1));
        let expected = Stack::new_from_vec(vec!['B', 'A', 'C'], 5);
        assert_eq!(expected, stack);
    }

    #[test]
    fn stack_yank_duplicate() {
        let mut stack = Stack::<char>::new(10);
        assert_eq!(0, stack.len());

        // Yanking an empty stack returns false
        assert_eq!(Err(ExecutionError::InsufficientInputs), stack.yank_duplicate(1));
        assert_eq!(Err(ExecutionError::InsufficientInputs), stack.yank_duplicate(500));
        assert_eq!(Err(ExecutionError::InsufficientInputs), stack.yank_duplicate(-1));

        // Yanking a single value is easy
        let mut stack = Stack::new_from_vec(vec!['A'], 5);
        assert_eq!(Ok(()), stack.yank_duplicate(1));
        let expected = Stack::new_from_vec(vec!['A', 'A'], 5);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['A'], 5);
        assert_eq!(Ok(()), stack.yank_duplicate(500));
        let expected = Stack::new_from_vec(vec!['A', 'A'], 5);
        assert_eq!(expected, stack);

        // Perform some more complicated yanks
        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.yank_duplicate(0));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A', 'A'], 5);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.yank_duplicate(1));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A', 'B'], 5);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.yank_duplicate(2));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A', 'C'], 5);
        assert_eq!(expected, stack);

        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.yank_duplicate(3));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A', 'A'], 5);
        assert_eq!(expected, stack);

        // Negative works too
        let mut stack = Stack::new_from_vec(vec!['C', 'B', 'A'], 5);
        assert_eq!(Ok(()), stack.yank_duplicate(-1));
        let expected = Stack::new_from_vec(vec!['C', 'B', 'A', 'C'], 5);
        assert_eq!(expected, stack);
    }
}
