use std::sync::Arc;

use derive_more::Display;
use serde::Deserialize;
use sqlx::prelude::FromRow;

use crate::driven::repository::Repository;

#[derive(sqlx::Type, Display, Clone, Deserialize)]
#[sqlx(type_name = "set_kind")]
pub enum SetKind {
    Normal,
    Myo,
    LengthenedPartial,
    Drop,
}

#[derive(Deserialize)]
pub struct NewLiftingLogEntryWithForeignEntityNames {
    pub rep_count: i64,
    pub set_kind: SetKind,
    pub rpe: i64,
    pub exercise: String,
    pub workout: String,
    pub routine: String,
}

#[derive(Clone, FromRow)]
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

#[derive(Clone)]
pub struct LiftingLogEntryWithForeignEntityNames {
    pub id: i64,
    pub rep_count: i64,
    pub set_kind: SetKind,
    pub rpe: i64,
    pub exercise: String,
    pub workout: String,
    pub routine: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct LiftingLogService {
    pub database_repository: Arc<dyn Repository + Send + Sync>,
}

impl<'a> LiftingLogService {
    pub async fn add_lifting_log_entry(
        &self,
        new_lifting_log_entry: NewLiftingLogEntryWithForeignEntityNames,
    ) -> LiftingLogEntry {
        self.database_repository
            .add_lifting_log_entry(new_lifting_log_entry)
            .await
    }

    pub async fn get_lifting_log_entry_with_foreign_entity_names_by_id(
        &self,
        lifting_log_entry_id: i64,
    ) -> LiftingLogEntryWithForeignEntityNames {
        self.database_repository
            .get_lifting_log_entry_with_foreign_entity_names_by_id(lifting_log_entry_id)
            .await
    }

    pub async fn get_lifting_log_entries(&self) -> Vec<LiftingLogEntry> {
        self.database_repository.get_lifting_log_entries().await
    }

    pub async fn get_lifting_log_entries_and_foreign_entity_names(
        &self,
    ) -> Vec<LiftingLogEntryWithForeignEntityNames> {
        self.database_repository
            .get_lifting_log_entries_and_foreign_entity_names()
            .await
    }

    pub async fn delete_lifting_log_entry(&self, lifting_log_id: i64) -> () {
        self.database_repository
            .delete_lifting_log_entry(lifting_log_id)
            .await
    }
}
