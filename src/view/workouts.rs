use std::collections::HashMap;

use leptos::*;
use uuid::Uuid;

use crate::data::Workout;

#[component]
pub fn ViewGlobalWorkoutsListToAddToRoutine(
    workouts: HashMap<Uuid, Workout>,
    routine_id: Uuid,
) -> impl IntoView {
    let workout_row_add_button = move |workout_id: Uuid| {
        view! {
            <AddworkoutToRoutineWorkoutsListButton workout_id=workout_id routine_id=routine_id></AddworkoutToRoutineWorkoutsListButton>
        }
    };
    let get_workout_row_component = move |workout: Workout| {
        view! {
            <ViewWorkoutsListRow workout=workout maybe_workout_row_add_button=Some(Box::new(workout_row_add_button)) maybe_workout_row_delete_button=None></ViewWorkoutsListRow>
        }
    };
    let get_workouts_add_button_component = move || {
        view! {
            <ButtonToNavigateToAddWorkoutFormForRoutineWorkouts routine_id=routine_id></ButtonToNavigateToAddWorkoutFormForRoutineWorkouts>
        }
    };
    view! {
        <ViewWorkoutsList workouts=workouts get_workout_row_component=Box::new(get_workout_row_component) get_workouts_add_button_component=Box::new(get_workouts_add_button_component) div_id="#content".to_owned()></ViewWorkoutsList>
    }
}

#[component]
pub fn ViewGlobalWorkoutsList(workouts: HashMap<Uuid, Workout>) -> impl IntoView {
    let workout_row_delete_button = move |workout_id: Uuid| {
        view! {
            <td>
                <DeleteWorkoutFromGlobalWorkoutsListButton workout_id=workout_id></DeleteWorkoutFromGlobalWorkoutsListButton>
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
            <AddWorkoutToGlobalWorkoutsListButton></AddWorkoutToGlobalWorkoutsListButton>
        }
    };
    view! {
        <ViewWorkoutsList workouts=workouts get_workout_row_component=Box::new(get_workout_row_component) get_workouts_add_button_component=Box::new(get_workouts_add_button_component) div_id="#content".to_owned()></ViewWorkoutsList>
    }
}

#[component]
pub fn ViewWorkoutsListForRoutine(
    workouts: HashMap<Uuid, Workout>,
    routine_id: Uuid,
) -> impl IntoView {
    let workout_row_delete_button = move |workout_id: Uuid| {
        view! {
            <DeleteWorkoutFromRoutineWorkoutsListButton workout_id=workout_id routine_id=routine_id></DeleteWorkoutFromRoutineWorkoutsListButton>
        }
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
    view! {
        <ViewWorkoutsList workouts=workouts get_workout_row_component=Box::new(get_workout_row_component) get_workouts_add_button_component=Box::new(get_workouts_add_button_component) div_id="#content".to_owned()></ViewWorkoutsList>
    }
}

#[component]
pub fn ViewWorkoutsList(
    workouts: HashMap<Uuid, Workout>,
    get_workout_row_component: Box<dyn Fn(Workout) -> View>,
    get_workouts_add_button_component: Box<dyn Fn() -> View>,
    div_id: String,
) -> impl IntoView {
    view! {
        <div id={div_id}>
            <p>Workouts:</p>
            <table>
                <thead>
                    <tr>
                    <th>ID</th>
                    <th>Name</th>
                    <th>Description</th>
                    <th></th>
                    <th></th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || workouts.clone()
                        key=|(id, _)| id.clone()
                        children=move |(_, workout): (Uuid, Workout)| {
                            get_workout_row_component(workout)
                        }
                    />
                    <tr>
                    <td>
                        { get_workouts_add_button_component() }
                    </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn ViewWorkoutsListRow(
    workout: Workout,
    maybe_workout_row_add_button: Option<Box<dyn Fn(Uuid) -> View>>,
    maybe_workout_row_delete_button: Option<Box<dyn Fn(Uuid) -> View>>,
) -> impl IntoView {
    let id = workout.id.to_string();
    view! {
        <tr>
        <td>{id.clone()}</td>
        <td>{workout.name}</td>
        <td>{workout.description}</td>
        <td>
            <button class="btn" hx-get={format!("/workouts/{id}")} hx-target="#content" hx-swap="outerHTML" hx-push-url="true">
            View
            </button>
        </td>
        {
            maybe_workout_row_add_button.map(|workout_row_add_button| workout_row_add_button(workout.id))
        }
        {
            maybe_workout_row_delete_button.map(|workout_row_delete_button| workout_row_delete_button(workout.id))
        }
        </tr>
    }
}

#[component]
fn ButtonToNavigateToAddWorkoutFormForRoutineWorkouts(routine_id: Uuid) -> impl IntoView {
    view! {
        <button class="btn" hx-get={format!("/routines/{routine_id}/workouts/add-workout-form")} hx-target="#content">
            Add Workouts to Routine
        </button>
    }
}

#[component]
fn AddWorkoutToGlobalWorkoutsListButton() -> impl IntoView {
    view! {
        <button class="btn" hx-post="/workouts/add-workout-form" hx-target="#content">
            Add Workout
        </button>
    }
}

#[component]
fn AddworkoutToRoutineWorkoutsListButton(routine_id: Uuid, workout_id: Uuid) -> impl IntoView {
    view! {
        <button class="btn" hx-post={format!("/routines/{routine_id}/workouts/{workout_id}", routine_id = routine_id.to_string(), workout_id = workout_id.to_string())} hx-target="closest tr" hx-swap="outerHTML">
        Add Workout to routine
        </button>
    }
}

#[component]
fn DeleteWorkoutFromRoutineWorkoutsListButton(workout_id: Uuid, routine_id: Uuid) -> impl IntoView {
    view! {
        <button class="btn" hx-delete={format!("/routines/{routine_id}/workouts/{workout_id}")} hx-target="closest tr" hx-swap="outerHTML">
            Delete Workout from Routine
        </button>
    }
}

#[component]
fn DeleteWorkoutFromGlobalWorkoutsListButton(workout_id: Uuid) -> impl IntoView {
    view! {
        <button class="btn" hx-delete={format!("/workouts/{workout_id}")} hx-target="closest tr" hx-swap="outerHTML">
            Delete Workout
        </button>
    }
}

#[component]
pub fn CreateWorkoutForm() -> impl IntoView {
    view! {
        <div id="content">
            <form hx-post="/workouts" hx-target="#content" hx-ext="json-enc" hx-swap="outerHTML">
                <input hidden name="id" value={Uuid::new_v4().to_string()} type="text"></input>
                <label>
                    Name
                    <input name="name" type="text"></input>
                </label>
                <label>
                    Description
                    <input name="description" type="text"></input>
                </label>
                <input type="submit"></input>
                <br></br>
            </form>
        </div>
    }
}
