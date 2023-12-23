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
    data,
    external::MyState,
    view::{
        head::Head,
        routines::{CreateRoutine, ViewRoutine, ViewRoutinesList},
    },
};

pub async fn view_routine(
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

pub async fn create_routine(
    my_state: State<Arc<Mutex<MyState>>>,
    maybe_routine: Option<Json<data::Routine>>,
) -> impl IntoResponse {
    maybe_routine.map_or(
        {
            let html = leptos::ssr::render_to_string(|| {
                view! {
                    <Head></Head>
                    <CreateRoutine></CreateRoutine>
                }
            })
            .to_string();

            (StatusCode::OK, Html(html))
        },
        |Json(routine)| {
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
        },
    )
}

pub async fn view_all_routines(State(my_state): State<Arc<Mutex<MyState>>>) -> impl IntoResponse {
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
