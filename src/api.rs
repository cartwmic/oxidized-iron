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
    view::{self, create_workout::CreateWorkout, workouts::Workouts},
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

pub async fn delete_routine(
    my_state: State<Arc<Mutex<MyState>>>,
    Path(routine_id): Path<Uuid>,
) -> impl IntoResponse {
    let mut inner = my_state.lock().unwrap();

    inner.routines.remove(&routine_id);

    StatusCode::OK
}

pub async fn delete_workout(
    my_state: State<Arc<Mutex<MyState>>>,
    Path(workout_id): Path<Uuid>,
) -> impl IntoResponse {
    let mut inner = my_state.lock().unwrap();

    inner.workouts.remove(&workout_id);

    StatusCode::OK
}

pub async fn get_routine(
    my_state: State<Arc<Mutex<MyState>>>,
    Path(routine_id): Path<Uuid>,
) -> impl IntoResponse {
    let inner = my_state.lock().unwrap();
    let routine = inner.routines.get(&routine_id).unwrap().clone();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <Head></Head>
            <view::routine::Routine routine=routine></view::routine::Routine>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}

pub async fn post_workout(
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
                    <Workouts workouts=workouts routine_id=None></Workouts>
                }
            })
            .to_string();

            (StatusCode::OK, Html(html))
        },
    )
}

pub async fn post_routine(
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

pub async fn get_workouts(State(my_state): State<Arc<Mutex<MyState>>>) -> impl IntoResponse {
    let inner = my_state.lock().unwrap();

    let workouts = inner.workouts.clone();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <Head></Head>
            <Workouts workouts=workouts routine_id=None></Workouts>
        }
    })
    .to_string();

    (StatusCode::OK, Html(html))
}
