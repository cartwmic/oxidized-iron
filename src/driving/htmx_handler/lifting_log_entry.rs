use std::sync::Arc;

use crate::{
    domain::lifting_log_entry::LiftingLogEntry,
    driving::htmx_handler::{format_id_to_htmx_target_, BASE_CONTENT_DIV_ID},
};
use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use leptos::*;
use tokio::sync::Mutex;

use super::{base, base_table, HtmxState, TableData};

#[debug_handler]
pub async fn get_lifting_log(State(htmx_state): State<Arc<Mutex<HtmxState>>>) -> impl IntoResponse {
    let inner = htmx_state.lock().await;
    let domain_service = &inner.domain_service;
    let lifting_log = domain_service.get_lifting_log_entries().await;

    let html = leptos::ssr::render_to_string(|| {
        base(view! {
            <ViewLiftingLog lifting_log=lifting_log></ViewLiftingLog>
        })
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

#[component]
pub fn ViewLiftingLogButton() -> impl IntoView {
    view! {
        <button hx-get="/lifting_log" hx-push-url="true" hx-target={ format_id_to_htmx_target_(BASE_CONTENT_DIV_ID.to_string()) }>View Lifting Log</button>
    }
}

#[component]
pub fn ViewLiftingLog(lifting_log: Vec<LiftingLogEntry>) -> impl IntoView {
    let table_data = TableData::new(lifting_log);
    base_table(
        table_data,
        "lifting-log-table".to_string(),
        "Lifting Log".to_string(),
    )
}
