use chrono::{DateTime, Utc};
use leptos::*;
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

#[derive(Serialize, Deserialize, Clone)]
pub struct Exercise {
    pub id: i64,
    pub name: String,
    pub implement: Implement,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Workout {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Routine {
    pub id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone, sqlx::Type, AsRefStr)]
#[sqlx(type_name = "implement")]
pub enum Implement {
    Bodyweight,
    Barbell,
    Dumbbell,
    MachineWeightStack,
    Cable,
    MachinePlateLoaded,
}

impl IntoView for Implement {
    fn into_view(self) -> leptos::View {
        view! {
            {self.as_ref().to_string()}
        }
        .into_view()
    }
}

#[derive(Serialize, Deserialize, Clone, sqlx::Type, AsRefStr)]
#[sqlx(type_name = "set_kind")]
pub enum SetKind {
    Normal,
    Myo,
    LengthenedPartial,
    Drop,
}

impl IntoView for SetKind {
    fn into_view(self) -> leptos::View {
        view! {
            {self.as_ref().to_string()}
        }
        .into_view()
    }
}

// for nested objects, should try https://www.reddit.com/r/rust/comments/1bm8vep/comment/kwdb9tj/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
