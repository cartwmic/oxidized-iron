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
    external::{get_reps, MyState},
    view::{base, ViewGlobalRepsList, ViewRepsButton},
};

pub fn get_router() -> Router<Arc<Mutex<MyState>>> {
    Router::new()
        .route("/", get(index))
        .route("/reps", get(get_reps_list))
}

pub async fn index() -> impl IntoResponse {
    let html = render_to_string(|| {
        base(
            view! {
                <ViewRepsButton></ViewRepsButton>
            }
            .into_view(),
        )
    })
    .to_string();
    (StatusCode::OK, Html(html))
}

pub async fn get_reps_list(State(my_state): State<Arc<Mutex<MyState>>>) -> impl IntoResponse {
    let inner = my_state.lock().await;
    let pool = &inner.database_connection_pool;

    let reps = get_reps(pool).await;

    let html = leptos::ssr::render_to_string(|| {
        base(view! {
            <ViewGlobalRepsList reps=reps></ViewGlobalRepsList>
        })
    })
    .to_string();

    (StatusCode::OK, Html(html))
}
