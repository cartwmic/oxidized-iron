use sqlx::{Pool, Postgres};

pub struct MyState {
    pub database_connection_pool: Pool<Postgres>,
}
