use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use leptos::{ssr::render_to_string, *};

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    external,
    external::{get_lifting_log_entries, MyState},
    view::{
        base, ViewExercises, ViewExercisesButton, ViewLiftingLog, ViewLiftingLogButton,
        ViewRoutines, ViewRoutinesButton, ViewWorkouts, ViewWorkoutsButton,
    },
};

pub fn get_router() -> Router<Arc<Mutex<MyState>>> {
    Router::new()
        .route("/", get(index))
        .route("/lifting_log", get(get_lifting_log))
        .route("/exercises", get(get_exercises))
        .route("/workouts", get(get_workouts))
        .route("/routines", get(get_routines))
}

pub async fn index() -> impl IntoResponse {
    let html = render_to_string(|| {
        base(
            view! {
                <ViewLiftingLogButton></ViewLiftingLogButton>
                <ViewExercisesButton></ViewExercisesButton>
                <ViewWorkoutsButton></ViewWorkoutsButton>
                <ViewRoutinesButton></ViewRoutinesButton>
            }
            .into_view(),
        )
    })
    .to_string();
    (StatusCode::OK, Html(html))
}

pub async fn get_lifting_log(State(my_state): State<Arc<Mutex<MyState>>>) -> impl IntoResponse {
    let inner = my_state.lock().await;
    let pool = &inner.database_connection_pool;

    let lifting_log = get_lifting_log_entries(pool).await;

    let html = leptos::ssr::render_to_string(|| {
        base(view! {
            <ViewLiftingLog lifting_log=lifting_log></ViewLiftingLog>
        })
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

pub async fn get_exercises(State(my_state): State<Arc<Mutex<MyState>>>) -> impl IntoResponse {
    let inner = my_state.lock().await;
    let pool = &inner.database_connection_pool;

    let exercises = external::get_exercises(pool).await;

    let html = leptos::ssr::render_to_string(|| {
        base(view! {
            <ViewExercises exercises=exercises></ViewExercises>
        })
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

pub async fn get_workouts(State(my_state): State<Arc<Mutex<MyState>>>) -> impl IntoResponse {
    let inner = my_state.lock().await;
    let pool = &inner.database_connection_pool;

    let workouts = external::get_workouts(pool).await;

    let html = leptos::ssr::render_to_string(|| {
        base(view! {
            <ViewWorkouts workouts=workouts></ViewWorkouts>
        })
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

pub async fn get_routines(State(my_state): State<Arc<Mutex<MyState>>>) -> impl IntoResponse {
    let inner = my_state.lock().await;
    let pool = &inner.database_connection_pool;

    let routines = external::get_routines(pool).await;

    let html = leptos::ssr::render_to_string(|| {
        base(view! {
            <ViewRoutines routines=routines></ViewRoutines>
        })
    })
    .to_string();

    (StatusCode::OK, Html(html))
}
