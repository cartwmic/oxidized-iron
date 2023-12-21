use axum::{
    extract::State,
    extract::{Json, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, post},
    Router,
};
use leptos::*;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let routines = HashMap::new();
    let my_state = Arc::new(Mutex::new(MyState { routines }));

    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/routines", get(get_routines))
        .route("/routines", post(post_routines))
        .route("/routines/:routine_id", delete(delete_routine))
        .with_state(my_state);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {
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

async fn delete_routine(
    my_state: State<Arc<Mutex<MyState>>>,
    Path(routine_id): Path<Uuid>,
) -> impl IntoResponse {
    let mut inner = my_state.lock().unwrap();

    inner.routines.remove(&routine_id);

    StatusCode::OK
}

async fn post_routines(
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

async fn get_routines(State(my_state): State<Arc<Mutex<MyState>>>) -> impl IntoResponse {
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

#[component]
fn CreateRoutine() -> impl IntoView {
    view! {
        <Head></Head>
        <div id="content">
            <form hx-post="/routines" hx-target="#content" hx-ext="json-enc" hx-swap="outerHTML">
                <input hidden name="id" value={Uuid::new_v4().to_string()} type="text"></input>
                <label>
                    Name
                    <input name="name" type="text"></input>
                </label>
                <label>
                    Description
                    <input name="description" type="text"></input>
                </label>
                <input type="submit"></input>
                <br></br>
            </form>
        </div>
    }
}

#[component]
fn Head() -> impl IntoView {
    view! {
        <head hx-ext="head-support" hx-head="merge">
            <script src="https://unpkg.com/htmx.org@1.9.6"
            integrity="sha384-FhXw7b6AlE/jyjlZH5iHa/tTe9EpJ1Y55RjcgPbjeWMskSxZt1v9qkxLJWNJaGni"
            crossorigin="anonymous"></script>
            <script src="https://unpkg.com/htmx.org/dist/ext/json-enc.js"></script>
            <script src="https://unpkg.com/htmx.org/dist/ext/head-support.js"></script>
        </head>
    }
}

#[component]
fn Routines(routines: HashMap<Uuid, Routine>) -> impl IntoView {
    view! {
    <Head></Head>
    <div id="content">
        <table>
            <thead>
                <tr>
                <th>ID</th>
                <th>Name</th>
                <th>Description</th>
                <th></th>
                <th></th>
                </tr>
            </thead>
            <tbody>
                <For
                    // a function that returns the items we're iterating over; a signal is fine
                    each=move || routines.clone()
                    // a unique key for each item
                    key=|(id, _)| id.clone()
                    // renders each item to a view
                    children=move |(id, routine): (Uuid, Routine)| {
                        let id = id.to_string();
                        view! {
                            <tr>
                            <td>{id.clone()}</td>
                            <td>{routine.name}</td>
                            <td>{routine.description}</td>
                            <td>
                                <button class="btn">
                                View
                                </button>
                            </td>
                            <td>
                                <button class="btn" hx-delete={format!("/routines/{id}")} hx-target="closest tr" hx-swap="outerHTML">
                                Delete
                                </button>
                            </td>
                            </tr>
                        }
                    }
                />
                <tr>
                <td>
                    <button class="btn" hx-post="/routines" hx-target="#content">
                    Add
                    </button>
                </td>
                </tr>
            </tbody>
        </table>
    </div>
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Routine {
    id: Uuid,
    name: String,
    description: String,
}

struct MyState {
    routines: HashMap<Uuid, Routine>,
}
