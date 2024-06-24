use std::sync::Arc;

use crate::{
    domain::exercise::Exercise,
    driving::htmx_handler::{format_id_to_htmx_target_, TableDataHeader, BASE_CONTENT_DIV_ID},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use leptos::*;
use tokio::sync::Mutex;

use super::{base, base_table, GetTableData, GetUrlPrefix, HtmxState, TableData};

pub async fn get_exercises(State(htmx_state): State<Arc<Mutex<HtmxState>>>) -> impl IntoResponse {
    let inner = htmx_state.lock().await;
    let domain_service = &inner.domain_service;
    let exercises = domain_service.get_exercises().await;

    let html = leptos::ssr::render_to_string(|| {
        base(view! {
            <ViewExercises exercises=exercises></ViewExercises>
        })
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

#[component]
pub fn ViewExercisesButton() -> impl IntoView {
    view! {
        <button hx-get="/exercises" hx-push-url="true" hx-target={ format_id_to_htmx_target_(BASE_CONTENT_DIV_ID.to_string()) }>View Exercises</button>
    }
}

#[component]
pub fn ViewExercises(exercises: Vec<Exercise>) -> impl IntoView {
    let table_data = TableData::new(exercises);
    base_table(
        table_data,
        "exercise-table".to_string(),
        "Exercises".to_string(),
    )
}

impl GetTableData for Exercise {
    fn get_table_data(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.to_string(),
            self.implement.to_string(),
            self.created_at.to_string(),
            self.updated_at.to_string(),
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
                header: "name".to_string(),
                value_is_editable: true,
            },
            TableDataHeader {
                header: "implement".to_string(),
                value_is_editable: true,
            },
            TableDataHeader {
                header: "created_at".to_string(),
                value_is_editable: false,
            },
            TableDataHeader {
                header: "updated_at".to_string(),
                value_is_editable: false,
            },
        ]
    }
}

impl GetUrlPrefix for Exercise {
    fn get_url_prefix() -> String {
        "exercises".to_string()
    }
}
