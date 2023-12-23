use axum::{
    extract::State,
    extract::{Json, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use leptos::*;

use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::{data, external::MyState, view::head::*, view::workouts::*};

pub async fn create_workout(
    my_state: State<Arc<Mutex<MyState>>>,
    maybe_workout: Option<Json<data::Workout>>,
) -> impl IntoResponse {
    maybe_workout.map_or(
        {
            let html = leptos::ssr::render_to_string(|| {
                view! {
                    <Head></Head>
                    <CreateWorkout></CreateWorkout>
                }
            })
            .to_string();

            (StatusCode::OK, Html(html))
        },
        |Json(workout)| {
            let mut inner = my_state.lock().unwrap();

            inner.workouts.insert(workout.id, workout);
            let workouts = inner.workouts.clone();

            let html = leptos::ssr::render_to_string(|| {
                view! {
                    <Head></Head>
                    <ViewWorkoutsList workouts=workouts maybe_routine_id=None></ViewWorkoutsList>
                }
            })
            .to_string();

            (StatusCode::OK, Html(html))
        },
    )
}

pub async fn get_workouts(State(my_state): State<Arc<Mutex<MyState>>>) -> impl IntoResponse {
    let inner = my_state.lock().unwrap();

    let workouts = inner.workouts.clone();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <Head></Head>
            <ViewWorkoutsList workouts=workouts maybe_routine_id=None></ViewWorkoutsList>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

pub async fn delete_workout(
    my_state: State<Arc<Mutex<MyState>>>,
    Path(workout_id): Path<Uuid>,
) -> impl IntoResponse {
    let mut inner = my_state.lock().unwrap();

    inner.workouts.remove(&workout_id);

    StatusCode::OK
}
