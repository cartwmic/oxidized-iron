use axum::Router;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::external::MyState;

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

    let app = Router::new()
        .merge(api::get_router())
        .merge(api::workouts::get_router())
        .merge(api::routines::get_router())
        .with_state(my_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
