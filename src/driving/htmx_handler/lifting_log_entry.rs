use std::sync::Arc;

use crate::{
    domain::lifting_log_entry::{
        LiftingLogEntryWithForeignEntityNames, NewLiftingLogEntryWithForeignEntityNames,
    },
    driving::htmx_handler::{format_id_to_htmx_target_, BASE_CONTENT_DIV_ID},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Form,
};
use leptos::*;
use tokio::sync::Mutex;

use super::{
    base, base_add_row, base_table, render_table_row, GetTableData, GetUrlPrefix, HtmxState,
    TableData, TableDataHeader,
};

pub async fn delete_lifting_log_entry(
    Path(lifting_log_id): Path<i64>,
    State(htmx_state): State<Arc<Mutex<HtmxState>>>,
) -> impl IntoResponse {
    let inner = htmx_state.lock().await;
    let domain_service = &inner.domain_service;
    domain_service
        .delete_lifting_log_entry(lifting_log_id)
        .await;

    let html = "".to_string();

    (StatusCode::OK, Html(html))
}

pub async fn add_lifting_log_entry(
    State(htmx_state): State<Arc<Mutex<HtmxState>>>,
    Form(new_lifting_log_entry): Form<NewLiftingLogEntryWithForeignEntityNames>,
) -> impl IntoResponse {
    let inner = htmx_state.lock().await;
    let domain_service = &inner.domain_service;
    let lifting_log_entry_id = domain_service
        .add_lifting_log_entry(new_lifting_log_entry)
        .await
        .id;
    let lifting_log_entry_with_foreign_entity_names = domain_service
        .get_lifting_log_entry_with_foreign_entity_names_by_id(lifting_log_entry_id)
        .await;

    let html = leptos::ssr::render_to_string(|| {
        render_table_row(lifting_log_entry_with_foreign_entity_names).into_view()
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

pub async fn get_lifting_log(State(htmx_state): State<Arc<Mutex<HtmxState>>>) -> impl IntoResponse {
    let inner = htmx_state.lock().await;
    let domain_service = &inner.domain_service;
    let lifting_log = domain_service.get_lifting_log_entries_for_table().await;

    let html = leptos::ssr::render_to_string(|| {
        base(view! {
            <ViewLiftingLogEntries lifting_log_entries=lifting_log></ViewLiftingLogEntries>
        })
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

pub async fn get_add_lifting_log_row() -> impl IntoResponse {
    let table_data: TableData<LiftingLogEntryWithForeignEntityNames> = TableData::new(vec![]);
    let html = leptos::ssr::render_to_string(|| base_add_row(table_data).into_view()).to_string();

    (StatusCode::OK, Html(html))
}

#[component]
pub fn ViewLiftingLogButton() -> impl IntoView {
    view! {
        <button hx-get="/lifting-log" hx-push-url="true" hx-target={ format_id_to_htmx_target_(BASE_CONTENT_DIV_ID.to_string()) }>View Lifting Log</button>
    }
}

#[component]
pub fn ViewLiftingLogEntries(
    lifting_log_entries: Vec<LiftingLogEntryWithForeignEntityNames>,
) -> impl IntoView {
    let table_data = TableData::new(lifting_log_entries);
    base_table(
        table_data,
        "lifting-log-table".to_string(),
        "Lifting Log".to_string(),
    )
}

impl GetTableData for LiftingLogEntryWithForeignEntityNames {
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

    fn get_headers() -> Vec<TableDataHeader> {
        vec![
            TableDataHeader {
                header: "id".to_string(),
                value_is_editable: false,
            },
            TableDataHeader {
                header: "rep_count".to_string(),
                value_is_editable: true,
            },
            TableDataHeader {
                header: "set_kind".to_string(),
                value_is_editable: true,
            },
            TableDataHeader {
                header: "rpe".to_string(),
                value_is_editable: true,
            },
            TableDataHeader {
                header: "exercise".to_string(),
                value_is_editable: true,
            },
            TableDataHeader {
                header: "workout".to_string(),
                value_is_editable: true,
            },
            TableDataHeader {
                header: "routine".to_string(),
                value_is_editable: true,
            },
            TableDataHeader {
                header: "created_at".to_string(),
                value_is_editable: false,
            },
        ]
    }
}

impl GetUrlPrefix for LiftingLogEntryWithForeignEntityNames {
    fn get_url_prefix() -> String {
        "lifting-log".to_string()
    }
}
