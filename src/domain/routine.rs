use std::sync::Arc;

use crate::driven::repository::Repository;

#[derive(Clone)]
pub struct Routine {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct RoutineService {
    pub database_repository: Arc<dyn Repository + Send + Sync>,
}

impl<'a> RoutineService {
    pub async fn get_routines(&self) -> Vec<Routine> {
        self.database_repository.get_routines().await
    }
}
