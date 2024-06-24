use std::sync::Arc;

use crate::{
    domain::workout::Workout,
    driving::htmx_handler::{format_id_to_htmx_target_, BASE_CONTENT_DIV_ID},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use leptos::*;
use tokio::sync::Mutex;

use super::{base, base_table, GetTableData, GetUrlPrefix, HtmxState, TableData, TableDataHeader};

pub async fn get_workouts(State(htmx_state): State<Arc<Mutex<HtmxState>>>) -> impl IntoResponse {
    let inner = htmx_state.lock().await;
    let domain_service = &inner.domain_service;
    let workouts = domain_service.get_workouts().await;

    let html = leptos::ssr::render_to_string(|| {
        base(view! {
            <ViewWorkouts workouts=workouts></ViewWorkouts>
        })
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

#[component]
pub fn ViewWorkoutsButton() -> impl IntoView {
    view! {
        <button hx-get="/workouts" hx-push-url="true" hx-target={ format_id_to_htmx_target_(BASE_CONTENT_DIV_ID.to_string()) }>View Workouts</button>
    }
}

#[component]
pub fn ViewWorkouts(workouts: Vec<Workout>) -> impl IntoView {
    let table_data = TableData::new(workouts);
    base_table(
        table_data,
        "workout-table".to_string(),
        "Workouts".to_string(),
    )
}

impl GetTableData for Workout {
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
                header: "description".to_string(),
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

impl GetUrlPrefix for Workout {
    fn get_url_prefix() -> String {
        "workouts".to_string()
    }
}
