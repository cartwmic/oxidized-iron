use std::sync::Arc;

use derive_more::Display;

use crate::{
    driven::repository::Repository,
    driving::htmx_handler::{GetTableData, GetUrlPrefix},
};

#[derive(sqlx::Type, Display, Clone)]
#[sqlx(type_name = "set_kind")]
pub enum SetKind {
    Normal,
    Myo,
    LengthenedPartial,
    Drop,
}

#[derive(Clone)]
pub struct LiftingLogEntry {
    pub id: i64,
    pub rep_count: i64,
    pub set_kind: SetKind,
    pub rpe: i64,
    pub exercise: i64,
    pub workout: i64,
    pub routine: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl GetTableData for LiftingLogEntry {
    fn get_table_data(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.rep_count.to_string(),
            self.set_kind.to_string(),
            self.rpe.to_string(),
            self.exercise.to_string(),
            self.workout.to_string(),
            self.routine.to_string(),
            self.created_at.to_string(),
        ]
    }

    fn get_data_id(&self) -> String {
        self.id.to_string()
    }

    fn get_headers() -> Vec<String> {
        vec![
            "id".to_string(),
            "rep_count".to_string(),
            "set_kind".to_string(),
            "rpe".to_string(),
            "exercise".to_string(),
            "workout".to_string(),
            "routine".to_string(),
            "created_at".to_string(),
        ]
    }

    fn get_human_readable_table_data(&self) -> Vec<String> {
        todo!() // trying to figure out dependency injection with hexagonal architecture before
                // moving on here
    }
}

impl GetUrlPrefix for LiftingLogEntry {
    fn get_url_prefix(&self) -> String {
        "lifting-log".to_string()
    }
}

pub struct LiftingLogService {
    pub database_repository: Arc<dyn Repository + Send + Sync>,
}

impl<'a> LiftingLogService {
    pub async fn get_lifting_log_entries(&self) -> Vec<LiftingLogEntry> {
        self.database_repository.get_lifting_log_entries().await
    }
}
