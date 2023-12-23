pub mod query_structs;
pub mod routines;
pub mod workouts;

use axum::{
    extract::State,
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use leptos::*;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use crate::{
    external::MyState,
    view::workouts::*,
    view::head::*,
};

use self::query_structs::WorkoutUuid;

pub async fn index() -> impl IntoResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
        <html lang="en">

        <Head></Head>

        <body>
            <h1>Oxidize RP</h1>
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

pub async fn post_routine_workout(
    my_state: State<Arc<Mutex<MyState>>>,
    Path(routine_id): Path<Uuid>,
    workout_uuid: Query<WorkoutUuid>,
) -> impl IntoResponse {
    let mut inner = my_state.lock().unwrap();
    let workouts = inner.workouts.clone();
    let workouts_2 = inner.workouts.clone();

    workout_uuid.maybe_workout_id.map_or(
        {
            let html = leptos::ssr::render_to_string(move || {
                view! {
                    <Head></Head>
                    <ViewWorkoutsListToAddToRoutine workouts=workouts routine_id=routine_id></ViewWorkoutsListToAddToRoutine>
                }
            })
            .to_string();

            (StatusCode::OK, Html(html)).into_response()
        },
        |workout_id| {
            inner
                .routines
                .get_mut(&routine_id)
                .unwrap()
                .workouts
                .get_or_insert(HashMap::new())
                .insert(
                    workout_id.clone(), 
                    workouts_2.get(&workout_id).unwrap().clone()
                );


            StatusCode::OK.into_response()
        },
    )
}

