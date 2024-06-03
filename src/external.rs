use sqlx::{Pool, Postgres};

use crate::data::Rep;
use crate::data::RepKind;

pub struct MyState {
    pub database_connection_pool: Pool<Postgres>,
}

pub async fn get_reps(pool: &Pool<Postgres>) -> Vec<Rep> {
    sqlx::query_as!(
        Rep,
        r#"
            SELECT id, kind AS "kind: RepKind", "set", notes, created_at, updated_at
                FROM data.rep
        "#,
    )
    .fetch_all(pool)
    .await
    .unwrap()
}
