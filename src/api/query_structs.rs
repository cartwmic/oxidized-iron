use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WorkoutUuid {
    pub maybe_workout_id: Option<i64>,
}
