#[derive(Debug)]
pub struct Stack<T> {
  stack: Vec<T>
}

impl<T> Stack<T> {
  pub fn new(items: Vec<T>) -> Self {
    Self {
      stack: items
    }
  }

  /// Push a new item ontop of the stack.
  pub fn push(&mut self, item: T) {
    self.stack.push(item);
  }

  /// Pop the item from the top of the stack.
  pub fn pop(&mut self) -> T {
    self.stack.pop().expect("tried to pop from stack when stack is empty")
  }
}