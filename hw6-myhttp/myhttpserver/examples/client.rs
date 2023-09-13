use reqwest::header::CONTENT_TYPE;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    // 测试 PING
    let pong = client
        .get("http://localhost:8000/ping")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(pong, "pong");

    let get = client
        .get("http://localhost:8000/ping/foo")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(get, "foo");
    // GET SET DEL
    let body = "key=foo&value=bar";
    let set = client
        .post("http://localhost:8000/set")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    assert_eq!(set.status(), 200);

    let get = client
        .get("http://localhost:8000/get/foo")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(get, "bar");

    let body = "key=foo";
    let del = client
        .post("http://localhost:8000/del")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    assert_eq!(del.status(), 200);

    let get = client
        .get("http://localhost:8000/get/foo")
        .send()
        .await
        .unwrap();
    // assert_eq!(get.status(), StatusCode::NOT_FOUND);
    assert_eq!(get.text().await.unwrap(), "Key not found!");

    // 测试 PUBLISH & SUBSCRIBE
    // let get = client
    //     .get("http://localhost:8000/subscribe/hobbitqia")
    //     .send()
    //     .await
    //     .unwrap()
    //     .text()
    //     .await;

    // let body = "key=hobbitqia&value=bar";
    // let set = client
    //     .post("http://localhost:8000/publish")
    //     .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
    //     .body(body)
    //     .send()
    //     .await
    //     .unwrap();
    // assert_eq!(set.status(), 200);
    // assert_eq!(set.text().await.unwrap(), "Subscibers: 1");
    // assert_eq!(get.unwrap(), "bar");


    println!("test success!");
}