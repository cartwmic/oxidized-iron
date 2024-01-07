use axum::Router;
use sqlx::postgres::PgPoolOptions;

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

#[derive(Debug)]
struct MyStruct {
    a: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // test sqlx
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/oxidized-iron")
        .await?;

    let row: Vec<MyStruct> = sqlx::query_as!(MyStruct, "SELECT 150 as a")
        .fetch_all(&pool)
        .await?;

    println!("{:?}", row);
    // end test sqlx

    let routines = HashMap::new();
    let workouts = HashMap::new();
    let my_state = Arc::new(Mutex::new(MyState { routines, workouts }));

    let app = Router::new()
        .merge(api::get_router())
        .merge(api::workouts::get_router())
        .merge(api::routines::get_router())
        .merge(api::routine_workouts::get_router())
        .with_state(my_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
