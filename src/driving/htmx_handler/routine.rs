use std::sync::Arc;

use crate::{
    domain::routine::Routine,
    driving::htmx_handler::{format_id_to_htmx_target_, BASE_CONTENT_DIV_ID},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use leptos::*;
use tokio::sync::Mutex;

use super::{base, base_table, HtmxState, TableData};

pub async fn get_routines(State(htmx_state): State<Arc<Mutex<HtmxState>>>) -> impl IntoResponse {
    let inner = htmx_state.lock().await;
    let domain_service = &inner.domain_service;
    let routines = domain_service.get_routines().await;

    let html = leptos::ssr::render_to_string(|| {
        base(view! {
            <ViewRoutines routines=routines></ViewRoutines>
        })
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

#[component]
pub fn ViewRoutinesButton() -> impl IntoView {
    view! {
        <button hx-get="/routine" hx-push-url="true" hx-target={ format_id_to_htmx_target_(BASE_CONTENT_DIV_ID.to_string()) }>View Lifting Log</button>
    }
}

#[component]
pub fn ViewRoutines(routines: Vec<Routine>) -> impl IntoView {
    let table_data = TableData::new(routines);
    base_table(
        table_data,
        "routine-table".to_string(),
        "Routines".to_string(),
    )
}
