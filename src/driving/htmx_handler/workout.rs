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

use super::{base, base_table, HtmxState, TableData};

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
        <button hx-get="/workout" hx-push-url="true" hx-target={ format_id_to_htmx_target_(BASE_CONTENT_DIV_ID.to_string()) }>View Lifting Log</button>
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
