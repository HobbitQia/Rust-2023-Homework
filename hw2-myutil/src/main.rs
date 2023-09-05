/*  main.rs
    测试 util.rs 中的函数。既使用了 assert! 语句，又通过 println! 语句可视化结果。
*/
mod util;

fn main() {
    println!("Hello, world!");
    test_my_buffer();
    test_my_compare();
    test_my_generate_new_vec();
}

fn test_my_buffer() {
    println!("测试 Buffer 类：");
    let tmp = util::Buffer::<i32>::new(vec![1, 2, 3, 4, 5]);
    assert!(tmp.sum().unwrap() == 15);
    println!("vec:{:?}, sum:{}", tmp.members, tmp.sum().unwrap());
    let tmp = util::Buffer::<i32>::new(vec![]);
    assert!(tmp.sum().is_none());
    println!("vec:{:?}, sum:{}", tmp.members, "None");
    let tmp = util::Buffer::<f32>::new(vec![0.6, 0.5, 0.0, -9.7, 90.0]);
    assert!(tmp.sum().unwrap() == 81.4);
    println!("vec:{:?}, sum:{}", tmp.members, tmp.sum().unwrap());
}

fn test_my_compare() {
    println!("测试 compare_string 函数：");
    let a = "def";
    let mut b = "abc";
    assert!(util::compare_string(a, b));
    println!("a:{}, b:{}, result:{}", "a", b, util::compare_string(a, b));     // true
    b = "deg";
    assert!(!util::compare_string(a, b));
    println!("a:{}, b:{}, result:{}", a, b, util::compare_string(a, b));     // false
    b = "ab";
    assert!(util::compare_string(a, b));
    println!("a:{}, b:{}, result:{}", a, b, util::compare_string(a, b));     // true
    b = "defghi";
    assert!(!util::compare_string(a, b));
    println!("a:{}, b:{}, result:{}", a, b, util::compare_string(a, b));     // false
    b = "def";
    assert!(!util::compare_string(a, b));
    println!("a:{}, b:{}, result:{}", a, b, util::compare_string(a, b));     // false
}

fn test_my_generate_new_vec() {
    println!("测试 generate_new_vec 函数：");
    let a = vec!['a', 'b', 'c', 'd', 'e'];
    println!("原：{:?}", a);
    assert!(util::generate_new_vec(&a) == vec!['b', 'c', 'd', 'e', 'f']);
    println!("新：{:?}", util::generate_new_vec(&a));
    let a = vec!['z', 'y', 'x'];
    println!("原：{:?}", a);
    assert!(util::generate_new_vec(&a) == vec!['a', 'z', 'y']);
    println!("新：{:?}", util::generate_new_vec(&a));
}