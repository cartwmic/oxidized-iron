use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct WorkoutUuid {
    pub maybe_workout_id: Option<Uuid>,
}
