/*  util.rs
    这个文件中包含了 Exercise 要求实现的主要功能。
*/

use std::iter::zip;

/*  Exercise 1 
    编写一个 Buffer 类，它包含一个 Vec<T>，实现 sum 方法，计算所有元素的和。
    如果 Vec<T> 为空，返回 None。
*/
pub struct Buffer<T> 
where T: std::ops::Add<Output = T> + Copy {
    pub members: Vec<T>
}

impl<T>Buffer<T> 
where T: std::ops::Add<Output = T> + Copy
    {
    pub fn new(members: Vec<T>) -> Self {
        Buffer::<T> {
            members
        }
    }
    pub fn sum(&self) -> Option<T>{
        if self.members.is_empty() {
            return None;
        }
        let mut ans = self.members[0];
        for i in 1..self.members.len() {
            ans = ans + self.members[i];
        }
        Some(ans)
    }
}

/*  Exercise 2
    编写一个函数 compare_string，比较两个字符串的大小，如果第一个字符串大于第二个字符串，返回 true，否则返回 false。
    如果两个字符串长度不同，我们认为较长的字符串大于较短的字符串。
*/
pub fn compare_string(x: &str, y: &str) -> bool {
    let vec_x: Vec<char> = x.chars().collect();
    let vec_y: Vec<char> = y.chars().collect();
    for (a, b) in zip(vec_x, vec_y) {
        if a > b {
            return true;
        }
        if a < b {
            return false;
        }
    }
    x.len() > y.len()
}

/*  Exercise 3
    编写一个函数 generate_new_vec，接受一个 Vec<char>，返回一个新的 Vec<char>，其中每个元素的值是原来的元素的下一个字符。
*/
pub fn generate_new_vec(a: &Vec<char>) -> Vec<char> {
    let f = |&x| { 
        if x == 'z' {
            return 'a';
        }
        (x as u8 + 1) as char
     };
    let iter = a.iter().map(f);
    iter.collect()
}
