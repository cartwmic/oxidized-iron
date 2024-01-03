use std::collections::HashMap;

use leptos::*;
use uuid::Uuid;

use crate::{data::Workout, view::workouts::*};

#[component]
pub fn ViewWorkoutsListForRoutine(
    workouts: HashMap<Uuid, Workout>,
    routine_id: Uuid,
) -> impl IntoView {
    let workout_row_delete_button = move |workout_id: Uuid| {
        view! {
            <td>
                <DeleteWorkoutFromRoutineWorkoutsListButton workout_id=workout_id routine_id=routine_id></DeleteWorkoutFromRoutineWorkoutsListButton>
            </td>
        }.into_view()
    };
    let get_workout_row_component = move |workout: Workout| {
        view! {
            <ViewWorkoutsListRow workout=workout maybe_workout_row_add_button=None maybe_workout_row_delete_button=Some(Box::new(workout_row_delete_button))></ViewWorkoutsListRow>
        }
    };
    let get_workouts_add_button_component = move || {
        view! {
            <ButtonToNavigateToAddWorkoutFormForRoutineWorkouts routine_id=routine_id></ButtonToNavigateToAddWorkoutFormForRoutineWorkouts>
        }
    };
    let get_workouts_back_button_component = move || view! {}.into_view();
    view! {
        <ViewWorkoutsList
            workouts=workouts
            get_workout_row_component=Box::new(get_workout_row_component)
            get_workouts_add_button_component=Box::new(get_workouts_add_button_component)
            div_id="content".to_owned()
            get_workouts_back_button_component=Box::new(get_workouts_back_button_component)>
        </ViewWorkoutsList>
    }
}
