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
    view::{create_routine::CreateRoutine, head::*, routines::Routines},
};

pub async fn index() -> impl IntoResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
        <html lang="en">

        <Head></Head>

        <body>
            <h1>Oxidize RP</h1>
            <div id="content">
                <button hx-get="/routines" hx-target="#content" hx-replace-url="true" hx-swap="outerHTML">View Routines</button>
            </div>
        </body>

        </html>
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

pub async fn post_routines(
    my_state: State<Arc<Mutex<MyState>>>,
    maybe_routine: Option<Json<Routine>>,
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
                    <Routines routines=routines></Routines>
                }
            })
            .to_string();

            (StatusCode::OK, Html(html))
        },
    )
}

pub async fn get_routines(State(my_state): State<Arc<Mutex<MyState>>>) -> impl IntoResponse {
    let inner = my_state.lock().unwrap();

    let routines = inner.routines.clone();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <Head></Head>
            <Routines routines=routines></Routines>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}
