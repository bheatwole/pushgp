use crate::util::stack_to_vec;

#[derive(Debug, PartialEq)]
pub struct Stack<T: Clone> {
    stack: Vec<T>,
}

// NOTE! Every Stack has a unique type, but not every stack has a Literal. For example, the Exec stack has the type
// of Exec which is an alias of Code, but neither Exec or Code have a literal.

impl<T: Clone> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            stack: vec![],
        }
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
    pub fn shove(&mut self, position: i64) {
        if self.stack.len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.len());
            let item = self.stack.pop().unwrap();
            self.stack.insert(vec_index, item);
        }
    }

    /// Reverses the position of the top two items on the stack. No effect if there are not at least two items.
    pub fn swap(&mut self) {
        if self.stack.len() >= 3 {
            let first = self.pop().unwrap();
            let second = self.pop().unwrap();
            self.push(first);
            self.push(second);
        }
    }

    /// Removes an item by its index from deep in the stack and pushes it onto the top. The position is taken modulus
    /// the original size of the stack. I.E. `yank(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `yank(2)` or `[ 'B', 'A', 'C' ]`.
    pub fn yank(&mut self, position: i64) {
        if self.stack.len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.len());
            let item = self.stack.remove(vec_index);
            self.stack.push(item);
        }
    }

    /// Copies an item by its index from deep in the stack and pushes it onto the top. The position is taken modulus
    /// the original size of the stack. I.E. `yank_duplicate(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `yank_duplicate(2)` or `[ 'C', 'B', 'A', 'C' ]`.
    pub fn yank_duplicate(&mut self, position: i64) {
        if self.stack.len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.len());
            let duplicate = self.stack.get(vec_index).unwrap().clone();
            self.stack.push(duplicate);
        }}
}
