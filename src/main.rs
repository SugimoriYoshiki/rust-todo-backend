mod handler;
mod repositories;

use crate::repositories::{TodoRepository, TodoRepositoryForMemory};
use handler::create_todo;

use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, Server,
};
use std::env;
use std::{net::SocketAddr, sync::Arc};

// TODO: フォルダ構成考えてみる(一般的なマイクローサービス系のやつ見てみる)
#[tokio::main]
async fn main() {
    // TODO: ここのログ冗長な気がする。もっとシンプルにできないものかね？あと環境変数にする。あと別ファイルにする
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let repository = TodoRepositoryForMemory::new();
    // TODO: testをドキュメントコメントと共に動かしてみる
    let app = create_app(repository);
    // TODO: 環境変数で管理できるようにする
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", addr); //ログの出力

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<T>))
        .layer(Extension(Arc::new(repository)))
}

async fn root() -> &'static str {
    "Hello, World!"
}

// テストやってみる
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "Hello, World!");
    }
}
