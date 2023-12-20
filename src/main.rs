use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use leptos::*;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(get_routines))
        .route("/routines", get(get_routines));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_routines() -> impl IntoResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <h1>"Hello, World!"</h1>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}
