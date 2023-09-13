
use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Form, Router,
};
use faststr::FastStr;
use my_redis::LogLayer;
use serde::Deserialize;
use volo_gen::my_redis::{ItemServiceClient, Item, ItemType};

// use volo_gen::my_redis::{DemoServiceClient, DemoServiceClientBuilder};
const DEFAULT_ADDR: &str = "127.0.0.1:8080";

#[derive(Deserialize, Debug)]
struct FormKey {
    key: String,
}

#[derive(Deserialize, Debug)]
struct FormKey2 {
    key: String,
    value: String,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = DEFAULT_ADDR.parse().unwrap();
    let rpc = volo_gen::my_redis::ItemServiceClientBuilder::new("my-redis")
            .layer_outer(LogLayer)
            .address(addr)
            .build();

    let app = Router::new()
        .route("/ping/:keys", get(ping)).with_state(rpc.clone())
        .route("/ping", get(ping_pong))
        .route("/get/:keys", get(get_key).with_state(rpc.clone()))
        .route("/set", get(show_set_form).post(set_key).with_state(rpc.clone()))
        .route("/del", get(show_del_form).post(del_key).with_state(rpc.clone()))
        .route("/subscribe/:keys", get(subscribe_key).with_state(rpc.clone()))
        .route("/publish", get(show_publish_form).post(publish_key).with_state(rpc));   


    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ping_pong() -> (StatusCode, &'static str) {
    (StatusCode::OK, "pong")
}

async fn ping(
    Path(key): Path<String>, 
    State(rpc_cli): State<ItemServiceClient>
) -> Response {
    let res = 
        rpc_cli.redis_command(
            Item{
                key: None,
                value: Some(FastStr::from(Arc::new(key))),
                request_type: ItemType::Ping,
                expire_time: None
            }
        ).await;
    match res {
        Ok(v) => {
            (StatusCode::OK, v.value.unwrap().to_string()).into_response()
        },
        Err(e) => {
            (StatusCode::NOT_FOUND, e.to_string()).into_response()
        }
    }
}

async fn get_key(
    Path(key): Path<String>, 
    State(rpc_cli): State<ItemServiceClient>
) -> Response {
    let res = 
        rpc_cli.redis_command(
            Item{
                key: Some(FastStr::from(Arc::new(key))),
                value: None,
                request_type: ItemType::Get,
                expire_time: None
            }
        ).await;
    match res {
        Ok(v) => {
            (StatusCode::OK, v.value.unwrap().to_string()).into_response()
        },
        Err(_) => {
            (StatusCode::NOT_FOUND, "Key not found!".to_string()).into_response()
        }
    }
}

async fn set_key(
    State(rpc_cli): State<ItemServiceClient>, 
    Form(setkey): Form<FormKey2>, 
) -> Response {
    let res = rpc_cli.redis_command(
        Item {
            key: Some(FastStr::from(Arc::new(setkey.key))),
            value: Some(FastStr::from(Arc::new(setkey.value))),
            // value: Some(FastStr::from(Arc::new(setvalue.key))),
            request_type: ItemType::Set,
            expire_time: None
        }
    ).await;
    match res {
        Ok(_) => {
            (StatusCode::OK, "Set Successfully!").into_response()
        },
        Err(e) => {
            (StatusCode::NOT_FOUND, e.to_string()).into_response()
        }
    }
}

async fn del_key(
    State(rpc_cli): State<ItemServiceClient>,
    Form(delkey): Form<FormKey>,
) -> (StatusCode, &'static str) {
    let res = rpc_cli.redis_command(
        Item {
            key: Some(FastStr::from(Arc::new(delkey.key))),
            value: None,
            request_type: ItemType::Del,
            expire_time: None
        }
    ).await;
    match res {
        Ok(_) => {
            (StatusCode::OK, "Delete Successfully!")
        },
        Err(_) => {
            (StatusCode::NOT_FOUND, "Delete Failed!")
        }
    }
}

async fn subscribe_key(
    Path(key): Path<String>, 
    State(rpc_cli): State<ItemServiceClient>
) -> Response {
    let res = rpc_cli.redis_command(
        Item {
            key: Some(FastStr::from(Arc::new(key))),
            value: None,
            request_type: ItemType::Subscribe,
            expire_time: None
        }
    ).await;
    match res {
        Ok(v) => {
            (StatusCode::OK, v.value.unwrap().to_string()).into_response()
        },
        Err(e) => {
            (StatusCode::NOT_FOUND, e.to_string()).into_response()
        }
    }
}

async fn publish_key(
    State(rpc_cli): State<ItemServiceClient>, 
    Form(publishkey): Form<FormKey2>, 
) -> Response {
    let res = rpc_cli.redis_command(
        Item {
            key: Some(FastStr::from(Arc::new(publishkey.key))),
            value: Some(FastStr::from(Arc::new(publishkey.value))),
            request_type: ItemType::Publish,
            expire_time: None
        }
    ).await;
    match res {
        Ok(v) => {
            (StatusCode::OK, v.value.unwrap().to_string()).into_response()
        },
        Err(e) => {
            (StatusCode::NOT_FOUND, e.to_string()).into_response()
        }
    }
}

async fn show_set_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/set" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <label for="key">
                        Enter value:
                    <input type="text" name="value">
                </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

async fn show_publish_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/publish" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <label for="key">
                        Enter value:
                    <input type="text" name="value">
                </label>
                    <input type="submit" value="Publish!">
                </form>
            </body>
        </html>
        "#,
    )
}

async fn show_del_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/del" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <input type="submit" value="Delete!">
                </form>
            </body>
        </html>
        "#,
    )
}