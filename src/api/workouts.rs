use axum::{
    extract::State,
    extract::{Json, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, post},
    Router,
};
use leptos::*;

use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::{data, external::MyState, view::head::*, view::workouts::*};

pub fn get_router() -> Router<Arc<Mutex<MyState>>> {
    Router::new()
        .route("/workouts", get(get_global_workouts_list_component))
        .route(
            "/workouts",
            post(add_workout_to_globabl_workouts_and_view_globabl_workouts_list_component),
        )
        .route(
            "/workouts/add-workout-form",
            get(get_component_for_adding_workout_to_globabl_workouts),
        )
        .route(
            "/workouts/:workout_id",
            delete(delete_workout_from_global_workouts),
        )
}

pub async fn add_workout_to_globabl_workouts_and_view_globabl_workouts_list_component(
    my_state: State<Arc<Mutex<MyState>>>,
    Json(workout): Json<data::Workout>,
) -> impl IntoResponse {
    let mut inner = my_state.lock().unwrap();

    inner.workouts.insert(workout.id, workout);
    let workouts = inner.workouts.clone();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <Head></Head>
            <ViewGlobalWorkoutsList workouts=workouts></ViewGlobalWorkoutsList>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

pub async fn get_component_for_adding_workout_to_globabl_workouts() -> impl IntoResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <Head></Head>
            <CreateWorkoutForm></CreateWorkoutForm>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

pub async fn get_global_workouts_list_component(
    State(my_state): State<Arc<Mutex<MyState>>>,
) -> impl IntoResponse {
    let inner = my_state.lock().unwrap();

    let workouts = inner.workouts.clone();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <Head></Head>
            <ViewGlobalWorkoutsList workouts=workouts></ViewGlobalWorkoutsList>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

pub async fn delete_workout_from_global_workouts(
    my_state: State<Arc<Mutex<MyState>>>,
    Path(workout_id): Path<Uuid>,
) -> impl IntoResponse {
    let mut inner = my_state.lock().unwrap();

    inner.workouts.remove(&workout_id);

    StatusCode::OK
}
