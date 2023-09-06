# HW1 - myfind

## 如何在本地构建使用

当前文件夹下已经有构建好的可执行文件 `hw1-myfind`，也可以通过下面的方式自行构建。

``` shell
$ git clone https://github.com/HobbitQia/Rust-2023-Homework.git
$ cd Rust-2023-Homework/hw1-myfind
$ cargo build # 也可以加上 --release 参数
```

随后可执行文件 `hw1-myfind` 可见在 `target/debug` 下。（如果加上了 `--release` 参数则可执行文件应在 `target/release` 下）

## 基本功能

* 可以搜出指定目录下符合正则表达式的文件路径。
* 支持输出所有遍历到的文件。
* 支持同时输入多个正则表达式。
* 支持同时搜索多个路径。
* 支持命令行彩色输出。

## 使用方式

终端命令：`<hw1-myfind的路径> [-n1|-n2] [-v|--verbose] <目标目录> <要搜索的正则表达式>`

* `[-n1|-n2]`
    * `-n1` 模式下，可以输入多个正则表达式，但只能输入一个路径。程序会返回同时满足所有正则表达式的文件路径。
    * `-n2` 模式下，可以输入多个文件路径，但只能输入一个正则表达式。程序会返回多个文件路径下满足这一正则表达式的文件路径。
* `[-v|--verbose]` (optional)
    * 可选，`-v` 和 `--verbose` 含义相同，会输出所有遍历到的文件。
* `<目标目录>`
    * 程序会以这个目录为根目录开始便利。
* `<要搜索的正则表达式>`

## 文件结构

``` shell
hw1-myfind
├─ src
│  ├─ helper            
│  │  ├─ mod.rs
│  │  └─ helper.rs      // 查找匹配文件
│  ├─ main.rs           // 实现 myfind 与终端交互
│  └─ myfind.rs         // 执行具体的 myfind 模式
├─ hw1-myfind           // 可执行文件
├─ Cargo.lock
├─ Cargo.toml
└─ README
```

## 示例

* `-n1` 模式  
    <div style="text-align: center; "><img src=https://cdn.hobbitqia.cc/%5D%7DOOSOCM1I7LGANPG7I%5DYNN.png width ="55%"></div>
    <div style="text-align: center; "><img src=https://cdn.hobbitqia.cc/JFJT5A%604H(@HG@J9JIDH%251K.png width ="55%"></div>
    <div style="text-align: center; "><img src=https://cdn.hobbitqia.cc/GS%7DAQTE~SP%25G4$52%7D8X79R9.png width ="55%"></div>

* `-n2` 模式
    <div style="text-align: center; "><img src=https://cdn.hobbitqia.cc/20230904213719.png width ="55%"></div>
    <div style="text-align: center; "><img src=https://cdn.hobbitqia.cc/XBLP9%7DN2X9F7EY%60_VP5NECY.png width ="55%"></div>
    <div style="text-align: center; "><img src=https://cdn.hobbitqia.cc/1.png width ="55%"></div>
