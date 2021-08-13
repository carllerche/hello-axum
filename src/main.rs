use axum::prelude::*;

#[tokio::main]
async fn main() {
    let app =
        route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(|| async { "OK" }));

    println!("AXUM - starting");

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
