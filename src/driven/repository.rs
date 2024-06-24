use axum::async_trait;

use crate::domain::{
    exercise::Exercise,
    lifting_log_entry::{
        LiftingLogEntry, LiftingLogEntryWithForeignEntityNames,
        NewLiftingLogEntryWithForeignEntityNames,
    },
    routine::Routine,
    workout::Workout,
};

#[async_trait]
pub trait Repository {
    async fn add_lifting_log_entry(
        &self,
        new_lifting_log_entry: NewLiftingLogEntryWithForeignEntityNames,
    ) -> LiftingLogEntry;

    async fn get_lifting_log_entries(&self) -> Vec<LiftingLogEntry>;

    async fn get_lifting_log_entry_with_foreign_entity_names_by_id(
        &self,
        lifting_log_entry_id: i64,
    ) -> LiftingLogEntryWithForeignEntityNames;

    async fn get_lifting_log_entries_and_foreign_entity_names(
        &self,
    ) -> Vec<LiftingLogEntryWithForeignEntityNames>;

    async fn delete_lifting_log_entry(&self, lifting_log_id: i64);

    async fn get_exercises(&self) -> Vec<Exercise>;

    async fn get_workouts(&self) -> Vec<Workout>;

    async fn get_routines(&self) -> Vec<Routine>;
}
