use axum::async_trait;
use sqlx::{Pool, Postgres};

use crate::{
    domain::{
        exercise::Exercise, exercise::Implement, lifting_log_entry::LiftingLogEntry,
        lifting_log_entry::SetKind, routine::Routine, workout::Workout,
    },
    driven::repository::Repository,
};

pub struct PostgresRepository {
    pub database_connection_pool: Box<Pool<Postgres>>,
}

#[async_trait]
impl Repository for PostgresRepository {
    async fn get_lifting_log_entries(&self) -> Vec<LiftingLogEntry> {
        sqlx::query_as!(
            LiftingLogEntry,
            r#"
                SELECT id, rep_count, set_kind AS "set_kind: SetKind", rpe, exercise, workout, ROUTINE
                      , created_at
                    FROM data.lifting_log_entry
        "#,
        )
        .fetch_all(&*self.database_connection_pool)
        .await
        .unwrap()
    }

    async fn get_exercises(&self) -> Vec<Exercise> {
        sqlx::query_as!(
            Exercise,
            r#"
                SELECT id, name, implement AS "implement: Implement", created_at, updated_at
                    FROM data.exercise
        "#,
        )
        .fetch_all(&*self.database_connection_pool)
        .await
        .unwrap()
    }

    async fn get_workouts(&self) -> Vec<Workout> {
        sqlx::query_as!(
            Workout,
            r#"
                SELECT *
                    FROM data.workout
        "#,
        )
        .fetch_all(&*self.database_connection_pool)
        .await
        .unwrap()
    }

    async fn get_routines(&self) -> Vec<Routine> {
        sqlx::query_as!(
            Routine,
            r#"
                SELECT *
                    FROM data.routine
        "#,
        )
        .fetch_all(&*self.database_connection_pool)
        .await
        .unwrap()
    }
}
