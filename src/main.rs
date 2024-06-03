use axum::Router;
use sqlx::postgres::PgPoolOptions;

use std::sync::Arc;
use tokio::sync::Mutex;

use oxidized_iron::{api::get_router, external::MyState};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/oxidized-iron")
        .await?;

    let my_state = Arc::new(Mutex::new(MyState {
        database_connection_pool,
    }));

    let app = Router::new().merge(get_router()).with_state(my_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
