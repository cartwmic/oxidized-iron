use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use leptos::{ssr::render_to_string, *};

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    external::{get_lifting_log_entries, MyState},
    view::{base, ViewLiftingLog, ViewLiftingLogButton},
};

pub fn get_router() -> Router<Arc<Mutex<MyState>>> {
    Router::new()
        .route("/", get(index))
        .route("/lifting_log", get(get_lifting_log))
}

pub async fn index() -> impl IntoResponse {
    let html = render_to_string(|| {
        base(
            view! {
                <ViewLiftingLogButton></ViewLiftingLogButton>
            }
            .into_view(),
        )
    })
    .to_string();
    (StatusCode::OK, Html(html))
}

pub async fn get_lifting_log(State(my_state): State<Arc<Mutex<MyState>>>) -> impl IntoResponse {
    let inner = my_state.lock().await;
    let pool = &inner.database_connection_pool;

    let lifting_log = get_lifting_log_entries(pool).await;

    let html = leptos::ssr::render_to_string(|| {
        base(view! {
            <ViewLiftingLog lifting_log=lifting_log></ViewLiftingLog>
        })
    })
    .to_string();

    (StatusCode::OK, Html(html))
}
