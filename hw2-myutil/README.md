# HW2 - myutil

## 如何在本地构建使用

当前文件夹下已经有构建好的可执行文件 `hw2-myutil`，也可以通过下面的方式自行构建。

```
$ git clone https://github.com/HobbitQia/Rust-2023-Homework.git
$ cd Rust-2023-Homework/hw2-myutil
$ cargo build   # 也可以加上 --release 参数
```

随后可执行文件 `hw2-myutil` 可见在 `target/debug` 下。（如果加上了 `--release` 参数则可执行文件应在 `target/release` 下）

## 基本功能

* Exercise 1：编写一个 Buffer 类，它包含一个 Vec<T>，实现 sum 方法，计算所有元素的和。
* Exercise 2：编写一个函数 compare_string，比较两个字符串的大小，如果第一个字符串大于第二个字符串，返回 true，否则返回 false。
* Exercise 3：编写一个函数 generate_new_vec，接受一个 Vec<char>，返回一个新的 Vec<char>，其中每个元素的值是原来的元素的下一个字符。

## 使用方式

直接运行可执行文件即可。

## 文件结构

```
hw2-myutil
├─ src
│  ├─ main.rs           // 测试 util.rs 中的函数
│  └─ util.rs           // 执行 Exercise 要求的主要内容
├─ hw2-myutil           // 可执行文件
├─ Cargo.lock
├─ Cargo.toml
└─ README
```

## 示例

![](https://cdn.hobbitqia.cc/20230905172219.png)