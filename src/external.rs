use sqlx::{Pool, Postgres};

use crate::data::*;

pub struct MyState {
    pub database_connection_pool: Pool<Postgres>,
}

pub async fn get_lifting_log_entries(pool: &Pool<Postgres>) -> Vec<LiftingLogEntry> {
    sqlx::query_as!(
        LiftingLogEntry,
        r#"
            SELECT id, rep_count, set_kind AS "set_kind: SetKind", rpe, exercise, workout, ROUTINE
                  , created_at
                FROM data.lifting_log_entry
        "#,
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

pub async fn get_exercises(pool: &Pool<Postgres>) -> Vec<Exercise> {
    sqlx::query_as!(
        Exercise,
        r#"
            SELECT id, name, implement AS "implement: Implement", created_at, updated_at
                FROM data.exercise
        "#,
    )
    .fetch_all(pool)
    .await
    .unwrap()
}
