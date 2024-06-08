use derive_more::Display;
use leptos::*;
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

use crate::{GetTableData, GetUrlPrefix};

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Exercise {
    pub id: i64,
    pub name: String,
    pub implement: Implement,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Workout {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Routine {
    pub id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Clone, sqlx::Type, AsRefStr, Hash, PartialEq, Eq, Display)]
#[sqlx(type_name = "implement")]
pub enum Implement {
    Bodyweight,
    Barbell,
    Dumbbell,
    MachineWeightStack,
    Cable,
    MachinePlateLoaded,
}

#[derive(Serialize, Deserialize, Clone, sqlx::Type, AsRefStr, Hash, PartialEq, Eq, Display)]
#[sqlx(type_name = "set_kind")]
pub enum SetKind {
    Normal,
    Myo,
    LengthenedPartial,
    Drop,
}

impl IntoView for Implement {
    fn into_view(self) -> leptos::View {
        view! {
            {self.as_ref().to_string()}
        }
        .into_view()
    }
}

impl IntoView for SetKind {
    fn into_view(self) -> leptos::View {
        view! {
            {self.as_ref().to_string()}
        }
        .into_view()
    }
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
}

impl GetUrlPrefix for LiftingLogEntry {
    fn get_url_prefix(&self) -> String {
        "lifting-log".to_string()
    }
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
}

impl GetUrlPrefix for Exercise {
    fn get_url_prefix(&self) -> String {
        "exercises".to_string()
    }
}

// for nested objects, should try https://www.reddit.com/r/rust/comments/1bm8vep/comment/kwdb9tj/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
