use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use askama::Template;
use axum::{
    extract::Json,
    extract::Path,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let routines = HashMap::new();
    let my_state = Arc::new(Mutex::new(MyState { routines }));

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/routine/:routine_id", put(edit_routine))
        .route("/routine/:routine_id/edit", get(edit_routine_html))
        .route("/routine/:routine_id/view", get(view_routine_html))
        .route("/routine/:routine_id", delete(delete_routine))
        .route("/routine/create", get(create_routine_html))
        .route("/routine", post(add_routine))
        .with_state(my_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(my_state): State<Arc<Mutex<MyState>>>) -> impl IntoResponse {
    let inner = my_state.lock().unwrap();

    let routines: Vec<&Routine> = inner.routines.values().into_iter().collect();

    let template = IndexTemplate { routines };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

async fn create_routine_html() -> impl IntoResponse {
    let template = CreateRoutine {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

async fn edit_routine_html(Path(routine_id): Path<Uuid>) -> impl IntoResponse {
    let template = EditRoutine { id: routine_id };
async fn view_routine_html(
    State(my_state): State<Arc<Mutex<MyState>>>,
    Path(routine_id): Path<Uuid>,
) -> impl IntoResponse {
    let state = my_state.lock().unwrap();
    let routine = state.routines.get(&routine_id).unwrap().clone();
    let template = ViewRoutine { routine };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

async fn edit_routine(
    State(my_state): State<Arc<Mutex<MyState>>>,
    Path(routine_id): Path<Uuid>,
    Json(routine): Json<Routine>,
) -> impl IntoResponse {
    let mut inner = my_state.lock().unwrap();
    inner.routines.insert(routine_id, routine.clone());

    let template = ViewRoutine { routine };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

async fn delete_routine(
    State(my_state): State<Arc<Mutex<MyState>>>,
    Path(routine_id): Path<Uuid>,
) -> impl IntoResponse {
    let mut inner = my_state.lock().unwrap();
    inner.routines.remove(&routine_id);

    let routines: Vec<&Routine> = inner.routines.values().into_iter().collect();

    let template = IndexTemplate { routines };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

async fn add_routine(
    State(my_state): State<Arc<Mutex<MyState>>>,
    Json(mut routine): Json<Routine>,
) -> impl IntoResponse {
    let new_routine_id = Uuid::new_v4();
    routine.id = new_routine_id;

    let mut inner = my_state.lock().unwrap();
    inner.routines.insert(routine.id, routine);

    let routines: Vec<&Routine> = inner.routines.values().into_iter().collect();

    let template = IndexTemplate { routines };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    routines: Vec<&'a Routine>,
}

#[derive(Template)]
#[template(path = "edit_routine.html")]
struct EditRoutine {
    id: Uuid,
}

#[derive(Template)]
#[template(path = "create_routine.html")]
struct CreateRoutine {}

#[derive(Serialize, Deserialize)]
struct Routine {
    id: Uuid,
    name: String,
    description: String,
}

struct MyState {
    routines: HashMap<Uuid, Routine>,
}
