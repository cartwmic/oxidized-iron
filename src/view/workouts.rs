use std::collections::HashMap;

use leptos::*;
use uuid::Uuid;

use crate::data::Workout;

#[component]
pub fn ViewWorkoutsListToAddToRoutine(
    workouts: HashMap<Uuid, Workout>,
    routine_id: Uuid,
) -> impl IntoView {
    view! {
        <div id="#content">
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
                        children=move |(id, workout): (Uuid, Workout)| {
                            let id_str = id.to_string();
                            view! {
                                <tr>
                                <td>{id_str.clone()}</td>
                                <td>{workout.name}</td>
                                <td>{workout.description}</td>
                                <td>
                                    <button class="btn" hx-post={format!("/routines/{routine_id}/workouts?maybe_workout_id={id}")} hx-target="closest tr" hx-swap="outerHTML">
                                    Add Workout to routine
                                    </button>
                                </td>
                                </tr>
                            }
                        }
                    />
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn ViewWorkoutsList(
    workouts: HashMap<Uuid, Workout>,
    maybe_routine_id: Option<Uuid>,
) -> impl IntoView {
    let mut div_id = "content";
    if let Some(_) = maybe_routine_id {
        div_id = "view-routine-workouts-list"
    }
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
                            view! {
                                <ViewWorkoutsListRow workout=workout maybe_routine_id=maybe_routine_id></ViewWorkoutsListRow>
                            }
                        }
                    />
                    <tr>
                    <td>
                        <AddWorkoutFromViewWorkoutsButton maybe_routine_id=maybe_routine_id></AddWorkoutFromViewWorkoutsButton>
                    </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn ViewWorkoutsListRow(workout: Workout, maybe_routine_id: Option<Uuid>) -> impl IntoView {
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
        <td>
            <DeleteWorkoutFromViewWorkoutsButton workout_id=workout.id maybe_routine_id=maybe_routine_id></DeleteWorkoutFromViewWorkoutsButton>
        </td>
        </tr>
    }
}

#[component]
fn AddWorkoutFromViewWorkoutsButton(maybe_routine_id: Option<Uuid>) -> impl IntoView {
    struct AddButtonVariationData {
        button_text: String,
        add_api_endpoint: String,
    }

    let mut addButtonVariationData = AddButtonVariationData {
        button_text: "Add Workout".to_owned(),
        add_api_endpoint: format!("/workouts"),
    };

    if let Some(routine_id) = maybe_routine_id {
        addButtonVariationData = AddButtonVariationData {
            button_text: "Add Workout to Routine".to_owned(),
            add_api_endpoint: format!("/routines/{routine_id}/workouts"),
        };
    };

    view! {
        <button class="btn" hx-post={addButtonVariationData.add_api_endpoint} hx-target="#content">
            { addButtonVariationData.button_text }
        </button>
    }
}

#[component]
fn DeleteWorkoutFromViewWorkoutsButton(
    workout_id: Uuid,
    maybe_routine_id: Option<Uuid>,
) -> impl IntoView {
    struct DeleteButtonVariationData {
        button_text: String,
        delete_api_endpoint: String,
    }

    let mut deleteButtonVariationData = DeleteButtonVariationData {
        button_text: "Delete Workout".to_owned(),
        delete_api_endpoint: format!("/workouts/{workout_id}"),
    };

    if let Some(routine_id) = maybe_routine_id {
        deleteButtonVariationData = DeleteButtonVariationData {
            button_text: "Delete Workout from Routine".to_owned(),
            delete_api_endpoint: format!("/routines/{routine_id}/workouts/{workout_id}"),
        };
    };

    view! {
        <button class="btn" hx-delete={deleteButtonVariationData.delete_api_endpoint} hx-target="closest tr" hx-swap="outerHTML">
            { deleteButtonVariationData.button_text }
        </button>
    }
}

#[component]
pub fn CreateWorkout() -> impl IntoView {
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
