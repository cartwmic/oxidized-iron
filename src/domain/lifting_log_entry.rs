use std::sync::Arc;

use derive_more::Display;

use crate::driven::repository::Repository;

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

pub struct LiftingLogService {
    pub database_repository: Arc<dyn Repository + Send + Sync>,
}

impl<'a> LiftingLogService {
    pub async fn get_lifting_log_entries(&self) -> Vec<LiftingLogEntry> {
        self.database_repository.get_lifting_log_entries().await
    }
}
