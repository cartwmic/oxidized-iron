pub mod query_structs;
pub mod routine_workouts;
pub mod routines;
pub mod workouts;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use leptos::*;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{external::MyState, view::index::Index};

pub fn get_router() -> Router<Arc<Mutex<MyState>>> {
    Router::new().route("/", get(index))
}

pub async fn index() -> impl IntoResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <Index></Index>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}
