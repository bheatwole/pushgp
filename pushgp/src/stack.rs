use crate::util::stack_to_vec;
use std::cell::RefCell;

/// Defines a stack of values that uses interior mutability for all operations
pub trait StackTrait<T: Clone> {
    /// Creates a new instance of the Stack
    fn new() -> Self;

    /// Returns the top item from the Stack or None if the stack is empty
    fn pop(&self) -> Option<T>;

    /// Pushes the specified item onto the top of the stack
    fn push(&self, item: T);

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

#[derive(Debug, PartialEq)]
pub struct Stack<T: Clone> {
    stack: RefCell<Vec<T>>,
}

// NOTE! Every Stack has a unique type, but not every stack has a Literal. For example, the Exec stack has the type
// of Exec which is an alias of Code, but neither Exec or Code have a literal.

impl<T: Clone> Stack<T> {
    pub fn new_from_vec(stack: Vec<T>) -> Stack<T> {
        Stack { stack: RefCell::new(stack) }
    }
}

impl<T: Clone> StackTrait<T> for Stack<T> {
    fn new() -> Stack<T> {
        Stack { stack: RefCell::new(vec![]) }
    }

    /// Returns the top item from the Stack or None if the stack is empty
    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }

    /// Pushes the specified item onto the top of the stack
    fn push(&self, item: T) {
        self.stack.borrow_mut().push(item)
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

#[cfg(test)]
mod tests {
    use crate::{Stack, StackTrait};

    #[test]
    fn stack_push_pop() {
        // Start with an empty stack
        let stack = Stack::<i32>::new();
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
        let stack = Stack::<i32>::new();
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
        let stack = Stack::<i32>::new();
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
        let stack = Stack::<i32>::new();
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
        let stack = Stack::<char>::new();
        assert_eq!(0, stack.len());

        // Shoving an empty stack returns zero
        assert_eq!(false, stack.shove(1));
        assert_eq!(false, stack.shove(500));
        assert_eq!(false, stack.shove(-1));

        // Shoving a single value succeeds but has no effect
        let stack = Stack::new_from_vec(vec!['A']);
        let expected = Stack::new_from_vec(vec!['A']);
        assert_eq!(true, stack.shove(1));
        assert_eq!(expected, stack);
        assert_eq!(true, stack.shove(500));
        assert_eq!(expected, stack);
        assert_eq!(true, stack.shove(-1));
        assert_eq!(expected, stack);

        // Perform some shoves
        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(0));
        assert_eq!(Stack::new_from_vec(vec!['C', 'B', 'A']), stack);

        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(1));
        assert_eq!(Stack::new_from_vec(vec!['C', 'A', 'B']), stack);

        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(2));
        assert_eq!(Stack::new_from_vec(vec!['A', 'C', 'B']), stack);

        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(3));
        assert_eq!(Stack::new_from_vec(vec!['C', 'B', 'A']), stack);

        // Negative works too
        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.shove(-1));
        assert_eq!(Stack::new_from_vec(vec!['A', 'C', 'B']), stack);
    }

    #[test]
    fn stack_swap() {
        let stack = Stack::<i32>::new();
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
        let stack = Stack::<char>::new();
        assert_eq!(0, stack.len());

        // Yanking an empty stack returns false
        assert_eq!(false, stack.yank(1));
        assert_eq!(false, stack.yank(500));
        assert_eq!(false, stack.yank(-1));

        // Yanking a single value succeeds but has no effect
        let stack = Stack::new_from_vec(vec!['A']);
        let expected = Stack::new_from_vec(vec!['A']);
        assert_eq!(true, stack.yank(1));
        assert_eq!(expected, stack);
        assert_eq!(true, stack.yank(500));
        assert_eq!(expected, stack);
        assert_eq!(true, stack.yank(-1));
        assert_eq!(expected, stack);

        // Perform some yanks
        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(0));
        assert_eq!(Stack::new_from_vec(vec!['C', 'B', 'A']), stack);

        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(1));
        assert_eq!(Stack::new_from_vec(vec!['C', 'A', 'B']), stack);

        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(2));
        assert_eq!(Stack::new_from_vec(vec!['B', 'A', 'C']), stack);

        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(3));
        assert_eq!(Stack::new_from_vec(vec!['C', 'B', 'A']), stack);

        // Negative works too
        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank(-1));
        assert_eq!(Stack::new_from_vec(vec!['B', 'A', 'C']), stack);
    }

    #[test]
    fn stack_yank_duplicate() {
        let stack = Stack::<char>::new();
        assert_eq!(0, stack.len());

        // Yanking an empty stack returns false
        assert_eq!(false, stack.yank_duplicate(1));
        assert_eq!(false, stack.yank_duplicate(500));
        assert_eq!(false, stack.yank_duplicate(-1));

        // Yanking a single value is easy
        let stack = Stack::new_from_vec(vec!['A']);
        assert_eq!(true, stack.yank_duplicate(1));
        assert_eq!(Stack::new_from_vec(vec!['A', 'A']), stack);

        let stack = Stack::new_from_vec(vec!['A']);
        assert_eq!(true, stack.yank_duplicate(500));
        assert_eq!(Stack::new_from_vec(vec!['A', 'A']), stack);

        // Perform some more complicated yanks
        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(0));
        assert_eq!(Stack::new_from_vec(vec!['C', 'B', 'A', 'A']), stack);

        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(1));
        assert_eq!(Stack::new_from_vec(vec!['C', 'B', 'A', 'B']), stack);

        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(2));
        assert_eq!(Stack::new_from_vec(vec!['C', 'B', 'A', 'C']), stack);

        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(3));
        assert_eq!(Stack::new_from_vec(vec!['C', 'B', 'A', 'A']), stack);

        // Negative works too
        let stack = Stack::new_from_vec(vec!['C', 'B', 'A']);
        assert_eq!(true, stack.yank_duplicate(-1));
        assert_eq!(Stack::new_from_vec(vec!['C', 'B', 'A', 'C']), stack);
    }
}
