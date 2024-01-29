use axum::{
    extract::Path,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, post},
    Router,
};
use leptos::*;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{data::Workout, external::MyState, view::head::*, view::workouts::*};

pub fn get_router() -> Router<Arc<Mutex<MyState>>> {
    Router::new()
        .route(
            "/routines/:routine_id/workouts/add-workout-form",
            get(get_component_for_adding_routine_to_workout),
        )
        .route(
            "/routines/:routine_id/workouts/:workout_id",
            post(add_workout_to_routine),
        )
        .route(
            "/routines/:routine_id/workouts/:workout_id",
            delete(delete_workout_from_routine),
        )
}

pub async fn get_component_for_adding_routine_to_workout(
    my_state: State<Arc<Mutex<MyState>>>,
    Path(routine_id): Path<i64>,
) -> impl IntoResponse {
    let inner = my_state.lock().await;
    let pool = &inner.database_connection_pool;

    let workouts: Vec<Workout> = sqlx::query_as!(
        Workout,
        r#"
            SELECT *
                FROM data.workout
        "#,
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let html = leptos::ssr::render_to_string(move || {
        view! {
            <Head></Head>
            <ViewGlobalWorkoutsListToAddToRoutine workouts=workouts routine_id=routine_id></ViewGlobalWorkoutsListToAddToRoutine>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html)).into_response()
}

pub async fn add_workout_to_routine(
    my_state: State<Arc<Mutex<MyState>>>,
    Path((routine_id, workout_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    let inner = my_state.lock().await;
    let pool = &inner.database_connection_pool;

    sqlx::query!(
        r#"
            INSERT INTO data.routine_workout (
                                      routine_id, workout_id)
                VALUES ($1, $2)
        "#,
        routine_id,
        workout_id,
    )
    .fetch_all(pool)
    .await
    .unwrap();

    StatusCode::OK.into_response()
}

pub async fn delete_workout_from_routine(
    my_state: State<Arc<Mutex<MyState>>>,
    Path((routine_id, workout_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    let inner = my_state.lock().await;
    let pool = &inner.database_connection_pool;

    sqlx::query!(
        r#"
            DELETE
                FROM data.routine_workout rw
                WHERE rw.routine_id = $1
                    AND rw.workout_id = $2
        "#,
        routine_id,
        workout_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    StatusCode::OK.into_response()
}
