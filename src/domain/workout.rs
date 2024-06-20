use std::sync::Arc;

use crate::driven::repository::Repository;

#[derive(Clone)]
pub struct Workout {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct WorkoutService {
    pub database_repository: Arc<dyn Repository + Send + Sync>,
}

impl<'a> WorkoutService {
    pub async fn get_workouts(&self) -> Vec<Workout> {
        self.database_repository.get_workouts().await
    }
}
