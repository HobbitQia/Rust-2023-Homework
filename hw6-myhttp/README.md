# HW6 - myhttpserver

## 简介

本次作业使用 Rust axum 搭建一个 HTTP server，server 内部调用我们 hw5 所写的 Mini-Redis，并且将 RPC 结果以 HTTP 方式返回给 client。  

同时，使用 reqwest 构造 HTTP client 请求，对 server 进行测试。

![](https://cdn.hobbitqia.cc/20230913201825.png)

## 如何在本地构建使用

可以通过下面的方式自行构建。

``` shell
$ git clone https://github.com/HobbitQia/Rust-2023-Homework.git
$ cd Rust-2023-Homework/hw6-myhttpserver
$ cd my-redis && cargo run --bin server            # 运行 Redis 服务端
$ cd ../myhttpserver && cargo run                  # 运行 HTTP 服务端
$ cargo run --example client                       # 运行 reqwest 测试代码
```
运行 Redis 服务器和 HTTP 服务器后，可以通过 `http://localhost:8000/[cmd]` 使用 HTTP 服务器的功能，具体 `cmd` 格式同 [hw5](https://github.com/HobbitQia/Rust-2023-Homework/tree/master/hw5-myredis)。

值得注意的是，本次作业中我为 `SET` `DEL` `PUBLISH` 命令设计了简易的 html 页面，需要手动输入信息。

* `PING`
    ``` bash
    PING [message]
    ```
    若不带参数，则返回 `PONG`；若带参数，则返回参数。
    ``` bash
    http://localhost:8000/
    PONG
    http://localhost:8000/Hello
    Hello
    ```
* `SET`
    ``` bash
    SET key value
    ```
    将 `key` 的值设为 `value`。若 `key` 已存在，则覆盖原值。  
    `http://localhost:8000/set` 即可跳转到输入界面，根据提示输入即可。具体示例见下一部分。
* `GET`
    ``` bash
    GET key
    ```
    返回 `key` 的值。若 `key` 不存在，则返回 `Key not found!`。
    ``` bash
    http://localhost:8000/get/foo
    $ ./server GET key_not_exist
    Key not found!
    ```
    这里我们如果已经把 `foo` 的值设为 2，那么会得到：
    ``` bash
    http://localhost:8000/get/foo
    2
    ```
* `DEL`
    ``` bash
    DEL key
    ```
    删除 `key`。若 `key` 不存在，则返回 `Some error happens: "Key not found!"`，否则返回 `OK!`。
    `http://localhost:8000/del/foo` 即可跳转到输入界面，根据提示输入即可。具体示例见下一部分。
* `SUBSCRIBE`
    ``` bash
    SUBSCRIBE channel
    ```
    > 由于框架 TCP 短连接的特性，客户端每次订阅只能获得一条消息。
    订阅 `channel`，等待下一条发送到 `channel` 的消息。
    ``` bash
    http://localhost:8000/subscribe/hobbitqia
                                                    # 此时网页会一直加载
    foo                                             # channel 被发布后会将对应的值显示
    ```
* `PUBLISH`
    ``` bash
    PUBLISH channel message
    ```
    向 `channel` 发送 `message`，并返回客户端数量。
    `http://localhost:8000/publish` 即可跳转到输入界面，根据提示输入即可。具体示例见下一部分。
* 中间件  
    在输入界面的 `value` 中输入带 `shabi` 的内容，将会得到 `application error: service error, msg: No dirty word, please!` 的信息。

## 文件结构

``` shell
hw6-myhttp/
├── README.md
├── myhttpserver/                       # HTTP 服务器
│       ├── examples
│       │   └── client.rs               # reqwest 测试文件
│       ├── src
│       │    └── main.rs                # http 服务器实现
│       ├── Cargo.lock
│       └── Cargo.toml
└── my-redis/                           # Mini-Redis
        ├── idl
        │   └── volo_example.thrift     # Thrift IDL 文件
        ├── src
        │    ├── lib.rs                 # 服务器端实现
        │    └── bin
        │         ├── client.rs         # 客户端程序
        │         └── server.rs         # 服务器程序
        ├── volo-gen
        │    ├── build.rs
        │    ├── Cargo.toml
        │    ├── src
        │    │    └── lib.rs
        │    └── volo.yml
        ├── Cargo.lock
        └── Cargo.toml
```

## 示例

* http 测试结果  
![](https://cdn.hobbitqia.cc/20230913195231.png)

* `PING` 测试结果  
![](https://cdn.hobbitqia.cc/20230913193815.png)  
![](https://cdn.hobbitqia.cc/20230913194100.png)

* `SET` 测试结果  
![](https://cdn.hobbitqia.cc/20230913194143.png)
![](https://cdn.hobbitqia.cc/20230913194159.png)
![](https://cdn.hobbitqia.cc/20230913194213.png)

* `GET` 测试结果  
![](https://cdn.hobbitqia.cc/20230913194234.png)

* `DEL` 测试结果  
![](https://cdn.hobbitqia.cc/20230913194301.png)  
![](https://cdn.hobbitqia.cc/20230913194808.png)
![](https://cdn.hobbitqia.cc/20230913194825.png)

* `PUBLISH` `SUBSCRIBE` 测试结果  
![](https://cdn.hobbitqia.cc/20230913194934.png)
![](https://cdn.hobbitqia.cc/20230913195124.png)  
![](https://cdn.hobbitqia.cc/20230913195219.png)
