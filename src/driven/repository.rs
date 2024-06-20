use axum::async_trait;

use crate::domain::{
    exercise::Exercise, lifting_log_entry::LiftingLogEntry, routine::Routine, workout::Workout,
};

#[async_trait]
pub trait Repository {
    async fn get_lifting_log_entries(&self) -> Vec<LiftingLogEntry>;

    async fn get_exercises(&self) -> Vec<Exercise>;

    async fn get_workouts(&self) -> Vec<Workout>;

    async fn get_routines(&self) -> Vec<Routine>;
}
