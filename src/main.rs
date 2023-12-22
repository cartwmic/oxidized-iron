use axum::{
    routing::{delete, get, post},
    Router,
};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::external::MyState;
use api::*;

pub mod api;
pub mod data;
pub mod external;
pub mod view;

#[tokio::main]
async fn main() {
    let routines = HashMap::new();
    let workouts = HashMap::new();
    let my_state = Arc::new(Mutex::new(MyState { routines, workouts }));

    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/routines", get(get_routines))
        .route("/workouts", get(get_workouts))
        .route("/routines", post(post_routine))
        .route("/workouts", post(post_workout))
        .route("/routines/:routine_id", delete(delete_routine))
        .route("/workouts/:workout_id", delete(delete_workout))
        .route("/routines/:routine_id", get(get_routine))
        .with_state(my_state);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
