# HW4 - myruntime

## 简介

本次作业使用 Rust 实现一个简单的单线程多任务 `Runtime`，并基于此为其添加多线程运行任务的能力。

设计主要分成以下几部分：

* `Waker`  
这里我们通过 `Waker` 来唤醒任务，且能做到在被唤醒前不 spin。简便起见这里采用了 `std` 提供的 `Condvar` 来实现这一功能。
* `Executor`
    * `TaskQueue` 用来存放要执行的任务，其中每个任务都由 `Task` 封装起来，`Task` 内包括一个 `Future` 和一个 `Signal`，用来实现多任务。
    * `Pool` 是一个线程调度池，其中包含一个工作线程的集合，用来实现多线程。


## 如何在本地构建使用

可以通过下面的方式自行构建。

``` shell
$ git clone https://github.com/HobbitQia/Rust-2023-Homework.git
$ cd Rust-2023-Homework/hw4-myfind
$ cargo run --example example1      # 测试 multitask
$ cargo run --example example2      # 测试 multithread
```

通过上面两种方式可以看到多任务、多线程的运行结果。

## 文件结构

``` shell
my_runtime/
├── src
│    ├── executor.rs    # 执行器
│    ├── lib.rs         # 导入 executor.rs 和 waker.rs 的内容
│    └── waker.rs       # 唤醒器
├── examples
│    ├── multitask.rs    # 测试多任务
│    └── multithread.rs  # 测试多线程
├── Cargo.lock
├── Cargo.toml
└── README
```

## 示例

单线程多任务：
![](https://cdn.hobbitqia.cc/20230909181834.png)

多线程：
![](https://cdn.hobbitqia.cc/20230909181931.png)

## 参考

* PPT 课件上的代码
* [茌海学长的分享](https://rustmagazine.github.io/rust_magazine_2021/chapter_12/monoio.html) 以及对应的[代码仓库](https://github.com/ihciah/mini-rust-runtime)
* [Rust 圣经中对 Future 底层逻辑的介绍](https://course.rs/advance/async/future-excuting.html)
* [博客：实现一个线程工作池 ThreadPool](https://www.cnblogs.com/linyihai/p/15885327.html)