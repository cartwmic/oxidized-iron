use axum::Router;
use sqlx::postgres::PgPoolOptions;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::external::MyState;

pub mod api;
pub mod data;
pub mod external;
pub mod utils;
pub mod view;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/oxidized-iron")
        .await?;

    let my_state = Arc::new(Mutex::new(MyState {
        database_connection_pool,
    }));

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
