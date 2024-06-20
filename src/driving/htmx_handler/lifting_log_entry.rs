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

use super::{base, base_table, GetTableData, GetUrlPrefix, HtmxState, TableData};

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

impl GetTableData for LiftingLogEntry {
    fn get_table_data(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.rep_count.to_string(),
            self.set_kind.to_string(),
            self.rpe.to_string(),
            self.exercise.to_string(),
            self.workout.to_string(),
            self.routine.to_string(),
            self.created_at.to_string(),
        ]
    }

    fn get_data_id(&self) -> String {
        self.id.to_string()
    }

    fn get_headers() -> Vec<String> {
        vec![
            "id".to_string(),
            "rep_count".to_string(),
            "set_kind".to_string(),
            "rpe".to_string(),
            "exercise".to_string(),
            "workout".to_string(),
            "routine".to_string(),
            "created_at".to_string(),
        ]
    }

    fn get_human_readable_table_data(&self) -> Vec<String> {
        self.get_table_data()
    }
}

impl GetUrlPrefix for LiftingLogEntry {
    fn get_url_prefix(&self) -> String {
        "lifting-log".to_string()
    }
}
