# HW3

文件结构如下：
``` shell
hw3
├─ src
│  ├─ main.rs           // 工程入口，输出提示信息
│  ├─ hashmap.rs        // Exercise 1 实现 HashMap 的宏以及测试代码
│  ├─ myrc.rs           // Exercise 2 实现引用计数 MyRc 以及测试代码
│  └─ lifo.rs           // Exercise 3 实现 LIFO 栈以及测试代码
├─ Cargo.lock
├─ Cargo.toml
└─ README
```

## Exercise 1

实现一个名为 `hash_map!` 的宏，它接受偶数个参数，并生成一个 `std::collections::HashMap`，其中第一个参数是键，第二个参数是值，以此类推。

在测试代码中，我们利用宏初始化定义了一个六个键值对的 `HashMap`，并检查了其中的键值是否对应正确。

## Exercise 2

实现一个简易的引用计数智能指针 `MyRc`，类似于 `std::rc::Rc`。

在测试代码中，我们创建了一个对象，并将其 `clone` 给另外两个对象。通过使用不同层次的大括号，我们让对象有不同的生命周期。在每对大括号中，我们检查了对象的引用计数以及对象的值是否正确。

## Exercise 3

实现一个简易的栈（LIFO）数据结构，支持 `push` 和 `pop` 操作，使用 `RefCell` 来实现内部可变性。

在测试代码中，我们创建了一个栈，并依次将 1、2、3 压入栈中，然后将 3、2 弹出栈。紧接着我们将 4 压入栈中，然后将 4、1 弹出栈。过程中我们检测栈的出栈元素是否正确，并在最后栈应为空时检测栈是否为空（即 `pop` 时应该得到 `None`）。

## 运行示例

构建方法：
``` shell
$ git clone https://github.com/HobbitQia/Rust-2023-Homework.git
$ cd Rust-2023-Homework/hw2-myutil
$ cargo test
```
得到测试结果如下：
``` shell
running 3 tests
test lifo::tests::test_lifo ... ok
test myrc::tests::test_myrc ... ok
test hashmap::tests::test_hash_map ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```