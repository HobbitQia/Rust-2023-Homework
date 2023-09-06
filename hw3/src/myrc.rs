/*  myrc.rs
    实现一个简单的引用计数智能指针，功能类似于 std::rc::Rc。  
    实现了 Deref trait，可以自动解引用；可以通过 strong_count 查看引用计数。
*/
use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::ops::Deref;

/*  RefCount
    引用计数结构体，包含一个计数器和一个对象。

*/
pub struct RefCount<T> {
    count: i32,
    object: T
}
impl<T> RefCount<T> {
    pub fn new(x: T) -> RefCount<T> {
        RefCount { count: 1, object: x } 
    }
    pub fn modify(&mut self, delta: i32) {
        self.count += delta;
    }
    /*  strong_count
        返回引用计数。
    */
    pub fn strong_count(&self) -> i32 { self.count }
}
impl<T> Drop for RefCount<T> {
    fn drop(&mut self) {
        println!("We are now dropping RefCount.");
    }
}
/*  MyRc
    引用计数智能指针，包含一个指向 RefCount 的指针。
*/
pub struct MyRc<T> {
    refcount: *mut RefCount<T>, 
}

impl<T> MyRc<T> {
    pub fn new(x: T) -> MyRc<T> {
        unsafe {                // 为了使用 alloc 和 dealloc，需要 unsafe
            let layout = Layout::new::<RefCount<T>>();
            let tmp = alloc(layout);
            if tmp.is_null() {
                handle_alloc_error(layout);
            }
            *(tmp as *mut RefCount<T>) = RefCount::new(x);
            MyRc{ refcount: tmp as *mut RefCount<T> }
        }
    } 
    /*  strong_count
        返回引用计数。
    */
    pub fn strong_count(&self) -> i32 { 
        unsafe {
            (*(self.refcount as *mut RefCount<T>)).strong_count() 
        }
    }
}

/*  Clone
    实现 Clone trait，可以通过 clone 方法复制 MyRc。
*/
impl<T> Clone for MyRc<T> {
    fn clone(&self) -> MyRc<T> {
        unsafe {
            (*(self.refcount as *mut RefCount<T>)).modify(1);
            MyRc { refcount:  self.refcount as *mut RefCount<T> }
        }
    }
}
/*  Drop
    实现 Drop trait，可以通过 drop 方法释放 MyRc。
*/
impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            (*(self.refcount as *mut RefCount<T>)).modify(-1);
            if (*(self.refcount as *mut RefCount<T>)).count == 0 {
                println!("We are now dropping MyRc.");
                let layout = Layout::new::<RefCount<T>>();
                dealloc(self.refcount as *mut u8, layout);
                
            }
        }
    }
}
/*  Deref
    实现 Deref trait，可以通过 * 解引用 MyRc。
*/
impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &(*(self.refcount as *mut RefCount<T>)).object
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_myrc() {
        let x = MyRc::new(88); {
            let y = MyRc::clone(&x); {
                let z = MyRc::clone(&x);
                assert_eq!(x.strong_count(), 3);
                assert_eq!(y.strong_count(), 3);
                assert_eq!(z.strong_count(), 3);
                assert_eq!(x.strong_count(), 3);

                assert_eq!(*x, 88);
                assert_eq!(*y, 88);
                assert_eq!(*z, 88);
            }
            assert_eq!(x.strong_count(), 2);
            assert_eq!(y.strong_count(), 2);

            assert_eq!(*x, 88);
            assert_eq!(*y, 88);
        }
        assert_eq!(x.strong_count(), 1);
        assert_eq!(*x, 88);
    }

}   

