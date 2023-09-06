/*  lifo.rs
    实现一个简单的后进先出（LIFO）栈，功能类似于 std::collections::VecDeque。
    使用 RefCell 来实现内部可变性。
*/
use std::cell::RefCell;

/*  MyLIFO
    后进先出（LIFO）栈。
*/
pub struct MyLIFO<T> {
    stack: RefCell<Vec<T>>
}

impl<T> MyLIFO<T>{
    pub fn new() -> MyLIFO<T> {
        MyLIFO{ stack: RefCell::new(Vec::new()) }
    }
    pub fn push(&self, x: T) -> () {
        self.stack.borrow_mut().push(x);
    }
    /* pop
        弹出栈顶元素。（通过 Option 返回）
        如果栈为空，返回 None。
    */
    pub fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifo() {
        let stack = MyLIFO::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        stack.push(4);
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

}   
