use axum::async_trait;
use sqlx::{Pool, Postgres};

use crate::{
    domain::{
        exercise::{Exercise, Implement},
        lifting_log_entry::{
            LiftingLogEntry, LiftingLogEntryWithForeignEntityNames,
            NewLiftingLogEntryWithForeignEntityNames, SetKind,
        },
        routine::Routine,
        workout::Workout,
    },
    driven::repository::Repository,
};

pub struct PostgresRepository {
    pub database_connection_pool: Box<Pool<Postgres>>,
}

#[async_trait]
impl Repository for PostgresRepository {
    async fn add_lifting_log_entry(
        &self,
        new_lifting_log_entry: NewLiftingLogEntryWithForeignEntityNames,
    ) -> LiftingLogEntry {
        let set_kind = new_lifting_log_entry.set_kind.to_string();
        sqlx::query_as!(
            LiftingLogEntry,
            r#"
                INSERT INTO data.lifting_log_entry (
                                          rep_count, set_kind, rpe, exercise, workout, ROUTINE)
                    SELECT $1, $2::data.set_kind, $3,
                        (SELECT id
                         FROM data.exercise
                         WHERE name = $4),
                        (SELECT id
                         FROM data.workout
                         WHERE name = $5),
                        (SELECT id
                         FROM data.routine
                         WHERE name = $6) RETURNING id, rep_count, set_kind AS "set_kind: SetKind", rpe, exercise, workout, ROUTINE
                      , created_at
            "#,
            new_lifting_log_entry.rep_count as i32,
            set_kind as String,
            new_lifting_log_entry.rpe as i32,
            new_lifting_log_entry.exercise as String,
            new_lifting_log_entry.workout as String,
            new_lifting_log_entry.routine as String,
        )
        .fetch_one(&*self.database_connection_pool)
        .await
        .unwrap()
    }

    async fn get_lifting_log_entry_with_foreign_entity_names_by_id(
        &self,
        lifting_log_entry_id: i64,
    ) -> LiftingLogEntryWithForeignEntityNames {
        sqlx::query_as!(
            LiftingLogEntryWithForeignEntityNames,
            r#"
                SELECT lle.id, rep_count, set_kind AS "set_kind: SetKind", rpe, e.name AS exercise
                      , w.name AS workout, r.name AS ROUTINE, lle.created_at
                    FROM data.lifting_log_entry lle
                    JOIN data.exercise e ON lle.exercise = e.id
                    JOIN data.workout w ON lle.workout = w.id
                    JOIN data.routine r ON lle.routine = r.id
                    WHERE lle.id = $1
            "#,
            lifting_log_entry_id
        )
        .fetch_one(&*self.database_connection_pool)
        .await
        .unwrap()
    }

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

    async fn get_lifting_log_entries_and_foreign_entity_names(
        &self,
    ) -> Vec<LiftingLogEntryWithForeignEntityNames> {
        sqlx::query_as!(
            LiftingLogEntryWithForeignEntityNames,
            r#"
                SELECT lle.id, rep_count, set_kind AS "set_kind: SetKind", rpe, e.name AS exercise
                      , w.name AS workout, r.name AS ROUTINE, lle.created_at
                    FROM data.lifting_log_entry lle
                    JOIN data.exercise e ON lle.exercise = e.id
                    JOIN data.workout w ON lle.workout = w.id
                    JOIN data.routine r ON lle.routine = r.id
            "#,
        )
        .fetch_all(&*self.database_connection_pool)
        .await
        .unwrap()
    }

    async fn delete_lifting_log_entry(&self, lifting_log_id: i64) -> () {
        sqlx::query!(
            r#"
                DELETE
                    FROM data.lifting_log_entry lle
                    WHERE lle.id = $1
            "#,
            lifting_log_id
        )
        .execute(&*self.database_connection_pool)
        .await
        .unwrap();
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
