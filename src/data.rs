use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Routine {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub workouts: Option<HashMap<Uuid, Workout>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Workout {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
