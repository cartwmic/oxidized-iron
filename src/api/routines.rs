use axum::{
    extract::State,
    extract::{Json, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use leptos::*;

use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::{
    data::Routine,
    external::MyState,
    view::{
        head::Head,
        routines::{CreateRoutineForm, ViewRoutine, ViewRoutinesList},
    },
};

pub async fn get_view_routine_component(
    my_state: State<Arc<Mutex<MyState>>>,
    Path(routine_id): Path<Uuid>,
) -> impl IntoResponse {
    let inner = my_state.lock().unwrap();
    let routine = inner.routines.get(&routine_id).unwrap().clone();

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
    Path(routine_id): Path<Uuid>,
) -> impl IntoResponse {
    let mut inner = my_state.lock().unwrap();

    inner.routines.remove(&routine_id);

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
    let mut inner = my_state.lock().unwrap();

    inner.routines.insert(routine.id, routine);
    let routines = inner.routines.clone();

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
    let inner = my_state.lock().unwrap();

    let routines = inner.routines.clone();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <Head></Head>
            <ViewRoutinesList routines=routines></ViewRoutinesList>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}
