pub mod query_structs;
pub mod routines;
pub mod workouts;

use axum::{
    extract::Path,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use leptos::*;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use crate::{external::MyState, view::head::*, view::workouts::*};

pub fn get_router() -> Router<Arc<Mutex<MyState>>> {
    Router::new()
        .route("/", get(index))
        .route(
            "/routines/:routine_id/workouts/add-workout-form",
            get(get_component_for_adding_routine_to_workout),
        )
        .route(
            "/routines/:routine_id/workouts/:workout_id",
            post(add_routine_to_workout),
        )
}

pub async fn index() -> impl IntoResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
        <html lang="en">

        <Head></Head>

        <body>
            <div id="content">
                <button hx-get="/routines" hx-target="#content" hx-push-url="true" hx-swap="outerHTML">View Routines</button>
                <button hx-get="/workouts" hx-target="#content" hx-push-url="true" hx-swap="outerHTML">View Workouts</button>
            </div>
        </body>

        </html>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

pub async fn get_component_for_adding_routine_to_workout(
    my_state: State<Arc<Mutex<MyState>>>,
    Path(routine_id): Path<Uuid>,
) -> impl IntoResponse {
    let inner = my_state.lock().unwrap();
    let workouts = inner.workouts.clone();

    let html = leptos::ssr::render_to_string(move || {
        view! {
            <Head></Head>
            <ViewGlobalWorkoutsListToAddToRoutine workouts=workouts routine_id=routine_id></ViewGlobalWorkoutsListToAddToRoutine>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html)).into_response()
}

pub async fn add_routine_to_workout(
    my_state: State<Arc<Mutex<MyState>>>,
    Path((routine_id, workout_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let mut inner = my_state.lock().unwrap();
    let workouts = inner.workouts.clone();

    inner
        .routines
        .get_mut(&routine_id)
        .unwrap()
        .workouts
        .get_or_insert(HashMap::new())
        .insert(
            workout_id.clone(),
            workouts.get(&workout_id).unwrap().clone(),
        );

    StatusCode::OK.into_response()
}
