use axum::{
    extract::State,
    extract::{Json, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, post},
    Router,
};
use leptos::*;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    data::Routine,
    data::Workout,
    external::MyState,
    view::{
        head::Head,
        routines::{CreateRoutineForm, ViewRoutine, ViewRoutinesList},
    },
};

pub fn get_router() -> Router<Arc<Mutex<MyState>>> {
    Router::new()
        .route("/routines", get(get_view_all_routines_component))
        .route(
            "/routines",
            post(create_routine_and_get_view_all_routines_component),
        )
        .route(
            "/routines/add-routine-form",
            get(get_create_routine_component),
        )
        .route("/routines/:routine_id", delete(delete_routine))
        .route("/routines/:routine_id", get(get_view_routine_component))
}

pub async fn get_view_routine_component(
    my_state: State<Arc<Mutex<MyState>>>,
    Path(routine_id): Path<i64>,
) -> impl IntoResponse {
    let inner = my_state.lock().await;
    let pool = &inner.database_connection_pool;

    let routine: Routine = sqlx::query_as!(
        Routine,
        r#"
            SELECT r.*, array_agg((w.*)) AS "workouts: Vec<Workout>"
                FROM data.routine r
                LEFT JOIN data.routine_workout rw ON r.id = rw.routine_id
                LEFT JOIN data.workout w ON rw.workout_id = w.id
                WHERE r.id = $1
                GROUP BY r.id
        "#,
        routine_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <Head></Head>
            <ViewRoutine routine=routine></ViewRoutine>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

pub async fn delete_routine(
    my_state: State<Arc<Mutex<MyState>>>,
    Path(routine_id): Path<i64>,
) -> impl IntoResponse {
    let inner = my_state.lock().await;
    let pool = &inner.database_connection_pool;

    sqlx::query_as!(
        Routine,
        r#"
            DELETE
                FROM data.routine r
                WHERE r.id = $1
        "#,
        routine_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    StatusCode::OK
}

pub async fn get_create_routine_component() -> impl IntoResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <Head></Head>
            <CreateRoutineForm></CreateRoutineForm>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

pub async fn create_routine_and_get_view_all_routines_component(
    my_state: State<Arc<Mutex<MyState>>>,
    Json(routine): Json<Routine>,
) -> impl IntoResponse {
    let inner = my_state.lock().await;
    let pool = &inner.database_connection_pool;

    sqlx::query_as!(
        Routine,
        r#"
            INSERT INTO data.routine (name, description)
                VALUES ($1, $2)
        "#,
        routine.name,
        routine.description
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let routines: Vec<Routine> = sqlx::query_as!(
        Routine,
        r#"
            SELECT *, NULL AS "workouts: Vec<Workout>"
                FROM data.routine
        "#,
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <Head></Head>
            <ViewRoutinesList routines=routines></ViewRoutinesList>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

pub async fn get_view_all_routines_component(
    State(my_state): State<Arc<Mutex<MyState>>>,
) -> impl IntoResponse {
    let inner = my_state.lock().await;
    let pool = &inner.database_connection_pool;

    let routines: Vec<Routine> = sqlx::query_as!(
        Routine,
        r#"
            SELECT *, NULL AS "workouts: Vec<Workout>"
                FROM data.routine
        "#,
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <Head></Head>
            <ViewRoutinesList routines=routines></ViewRoutinesList>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}
