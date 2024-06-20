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

use super::{base, base_table, GetTableData, GetUrlPrefix, HtmxState, TableData};

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
        <button hx-get="/routines" hx-push-url="true" hx-target={ format_id_to_htmx_target_(BASE_CONTENT_DIV_ID.to_string()) }>View Routines</button>
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

impl GetTableData for Routine {
    fn get_table_data(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.to_string(),
            self.description
                .as_ref()
                .get_or_insert(&"".to_string())
                .to_string(),
            self.created_at.to_string(),
            self.updated_at.to_string(),
        ]
    }

    fn get_data_id(&self) -> String {
        self.id.to_string()
    }

    fn get_headers() -> Vec<String> {
        vec![
            "id".to_string(),
            "name".to_string(),
            "description".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
        ]
    }

    fn get_human_readable_table_data(&self) -> Vec<String> {
        self.get_table_data()
    }
}

impl GetUrlPrefix for Routine {
    fn get_url_prefix(&self) -> String {
        "routines".to_string()
    }
}
