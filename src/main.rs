use axum::{
    routing::{delete, get, post},
    Router,
};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::external::MyState;
use api::{routines::*, workouts::*, *};

pub mod api;
pub mod data;
pub mod external;
pub mod utils;
pub mod view;

#[tokio::main]
async fn main() {
    let routines = HashMap::new();
    let workouts = HashMap::new();
    let my_state = Arc::new(Mutex::new(MyState { routines, workouts }));

    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/routines", get(view_all_routines))
        .route("/workouts", get(get_global_workouts_list_component))
        .route("/routines", post(create_routine))
        .route("/workouts", post(add_workout_to_globabl_workouts))
        .route("/routines/:routine_id", delete(delete_routine))
        .route(
            "/workouts/:workout_id",
            delete(delete_workout_from_global_workouts),
        )
        .route("/routines/:routine_id", get(view_routine))
        .route(
            "/routines/:routine_id/workouts/add-workout-form",
            get(get_component_for_adding_routine_to_workout),
        )
        .route(
            "/routines/:routine_id/workouts/:workout_id",
            post(add_routine_to_workout),
        )
        .with_state(my_state);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
