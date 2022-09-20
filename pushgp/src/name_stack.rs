use crate::{Name, Stack};

#[derive(Clone, Debug, PartialEq)]
pub struct NameStack {
    stack: Stack<Name>,
    quote_next_name: bool,
}

impl NameStack {
    pub fn new(max_len: usize) -> NameStack {
        NameStack { stack: Stack::new(max_len), quote_next_name: false }
    }

    pub fn clear(&mut self) {
        self.stack.clear();
        self.quote_next_name = false;
    }

    pub fn should_quote_next_name(&self) -> bool {
        self.quote_next_name
    }

    pub fn set_should_quote_next_name(&mut self, quote_next_name: bool) {
        self.quote_next_name = quote_next_name;
    }

    /// Returns the top item from the Stack or None if the stack is empty
    pub fn pop(&mut self) -> Option<Name> {
        self.stack.pop()
    }

    /// Returns a clone of the top item from the Stack or None if the stack is empty
    pub fn peek(&self) -> Option<Name> {
        self.stack.peek()
    }

    /// Pushes the specified item onto the top of the stack
    pub fn push(&mut self, item: Name) {
        self.stack.push(item)
    }

    /// Returns the length of the Stack
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Duplicates the top item of the stack. This should not change the Stack or panic if the stack is empty
    pub fn duplicate_top_item(&mut self) {
        self.stack.duplicate_top_item()
    }

    /// Rotates the top three items on the stack, pulling the third item out and pushing it on top. This should not
    /// modify the stack if there are fewer than three items
    pub fn rotate(&mut self) {
        self.stack.rotate()
    }

    /// Pops the top item of the stack and pushes it down the specified number of positions. Thus `shove(0)` has no
    /// effect. The position is taken modulus the original size of the stack. I.E. `shove(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `shove(2)` or `[ 'A', 'C', 'B' ]`.
    ///
    /// Returns true if a shove was performed (even if it had no effect)
    pub fn shove(&mut self, position: i64) -> bool {
        self.stack.shove(position)
    }

    /// Reverses the position of the top two items on the stack. No effect if there are not at least two items.
    pub fn swap(&mut self) {
        self.stack.swap()
    }

    /// Removes an item by its index from deep in the stack and pushes it onto the top. The position is taken modulus
    /// the original size of the stack. I.E. `yank(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `yank(2)` or `[ 'B', 'A', 'C' ]`.
    ///
    /// Returns true if a yank was performed (even if it had no effect)
    pub fn yank(&mut self, position: i64) -> bool {
        self.stack.yank(position)
    }

    /// Copies an item by its index from deep in the stack and pushes it onto the top. The position is taken modulus
    /// the original size of the stack. I.E. `yank_duplicate(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `yank_duplicate(2)` or `[ 'C', 'B', 'A', 'C' ]`.
    ///
    /// Returns true if a yank was performed (even if it had no effect)
    pub fn yank_duplicate(&mut self, position: i64) -> bool {
        self.stack.yank_duplicate(position)
    }
}
