use std::sync::Arc;

use derive_more::Display;

use crate::driven::repository::Repository;

#[derive(sqlx::Type, Display, Clone)]
pub enum Implement {
    Bodyweight,
    Barbell,
    Dumbbell,
    MachineWeightStack,
    Cable,
    MachinePlateLoaded,
}

#[derive(Clone)]
pub struct Exercise {
    pub id: i64,
    pub name: String,
    pub implement: Implement,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct ExerciseService {
    pub database_repository: Arc<dyn Repository + Send + Sync>,
}

impl<'a> ExerciseService {
    pub async fn get_exercises(&self) -> Vec<Exercise> {
        self.database_repository.get_exercises().await
    }
}
