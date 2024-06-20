use std::sync::Arc;

use crate::{
    domain::exercise::Exercise,
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
        <button hx-get="/exercise" hx-push-url="true" hx-target={ format_id_to_htmx_target_(BASE_CONTENT_DIV_ID.to_string()) }>View Lifting Log</button>
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
