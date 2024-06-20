use std::sync::Arc;

use derive_more::Display;

use crate::{
    driven::repository::Repository,
    driving::htmx_handler::{GetTableData, GetUrlPrefix},
};

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

impl GetTableData for Exercise {
    fn get_table_data(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.to_string(),
            self.implement.to_string(),
            self.created_at.to_string(),
            self.updated_at.to_string(),
        ]
    }

    fn get_data_id(&self) -> String {
        self.id.to_string()
    }

    fn get_headers() -> Vec<String> {
        vec![
            "id".to_string(),
            "name".to_string(),
            "implement".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
        ]
    }

    fn get_human_readable_table_data(&self) -> Vec<String> {
        self.get_table_data()
    }
}

impl GetUrlPrefix for Exercise {
    fn get_url_prefix(&self) -> String {
        "exercises".to_string()
    }
}

pub struct ExerciseService {
    pub database_repository: Arc<dyn Repository + Send + Sync>,
}

impl<'a> ExerciseService {
    pub async fn get_exercises(&self) -> Vec<Exercise> {
        self.database_repository.get_exercises().await
    }
}
