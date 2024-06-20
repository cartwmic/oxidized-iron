use std::sync::Arc;

use crate::{
    driven::repository::Repository,
    driving::htmx_handler::{GetTableData, GetUrlPrefix},
};

#[derive(Clone)]
pub struct Routine {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl GetTableData for Routine {
    fn get_table_data(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.to_string(),
            self.description
                .as_ref()
                .get_or_insert(&"".to_string())
                .to_string(),
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
            "description".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
        ]
    }

    fn get_human_readable_table_data(&self) -> Vec<String> {
        self.get_table_data()
    }
}

impl GetUrlPrefix for Routine {
    fn get_url_prefix(&self) -> String {
        "routines".to_string()
    }
}

pub struct RoutineService {
    pub database_repository: Arc<dyn Repository + Send + Sync>,
}

impl<'a> RoutineService {
    pub async fn get_routines(&self) -> Vec<Routine> {
        self.database_repository.get_routines().await
    }
}
