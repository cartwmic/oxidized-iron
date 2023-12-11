use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, patch, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/routine/:routine_id", put(edit_routine))
        .route("/routine/:routine_id", delete(delete_routine))
        .route("/routine", post(add_routine));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> impl IntoResponse {
    let template = IndexTemplate {
        routines: vec![
            Routine { id: Uuid::new_v4() },
            Routine { id: Uuid::new_v4() },
        ],
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

async fn edit_routine(Path(routine_id): Path<Uuid>) -> impl IntoResponse {
    let template = IndexTemplate {
        routines: vec![
            Routine { id: Uuid::new_v4() },
            Routine { id: Uuid::new_v4() },
        ],
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

async fn delete_routine(Path(routine_id): Path<Uuid>) -> impl IntoResponse {
    let template = IndexTemplate {
        routines: vec![
            Routine { id: Uuid::new_v4() },
            Routine { id: Uuid::new_v4() },
        ],
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

async fn add_routine() -> impl IntoResponse {
    let template = IndexTemplate {
        routines: vec![
            Routine { id: Uuid::new_v4() },
            Routine { id: Uuid::new_v4() },
        ],
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    routines: Vec<Routine>,
}

#[derive(Serialize, Deserialize)]
struct Routine {
    id: Uuid,
}
