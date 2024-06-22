pub mod exercise;
pub mod lifting_log_entry;
pub mod routine;
pub mod workout;

// for nested objects, should try https://www.reddit.com/r/rust/comments/1bm8vep/comment/kwdb9tj/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

use exercise::{Exercise, ExerciseService};
use lifting_log_entry::{
    LiftingLogEntry, LiftingLogEntryWithForeignEntityNames, LiftingLogService,
};
use routine::{Routine, RoutineService};
use workout::{Workout, WorkoutService};

pub struct DomainService {
    pub lifting_log_entry_service: LiftingLogService,
    pub exercise_service: ExerciseService,
    pub routine_service: RoutineService,
    pub workout_service: WorkoutService,
}

impl<'a> DomainService {
    pub async fn get_lifting_log_entries(&self) -> Vec<LiftingLogEntry> {
        self.lifting_log_entry_service
            .get_lifting_log_entries()
            .await
    }

    pub async fn get_lifting_log_entries_for_table(
        &self,
    ) -> Vec<LiftingLogEntryWithForeignEntityNames> {
        self.lifting_log_entry_service
            .get_lifting_log_entries_and_foreign_entity_names()
            .await
    }

    pub async fn get_exercises(&self) -> Vec<Exercise> {
        self.exercise_service.get_exercises().await
    }

    pub async fn get_routines(&self) -> Vec<Routine> {
        self.routine_service.get_routines().await
    }

    pub async fn get_workouts(&self) -> Vec<Workout> {
        self.workout_service.get_workouts().await
    }
}
