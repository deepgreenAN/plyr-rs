#[tokio::main]
async fn main() {
    use axum::{http::StatusCode, routing::get_service, Router};
    use tower_http::services::{ServeDir, ServeFile};

    let app: Router<()> = Router::new()
        .route(
            "/",
            get_service(ServeFile::new("../dist/index.html"))
                .handle_error(|_| async move { StatusCode::NOT_FOUND }),
        )
        .nest_service(
            "/dist",
            get_service(ServeDir::new("../dist"))
                .handle_error(|_| async move { StatusCode::NOT_FOUND }),
        )
        .nest_service(
            "/assets",
            get_service(ServeDir::new("../assets"))
                .handle_error(|_| async move { StatusCode::NOT_FOUND }),
        );

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
