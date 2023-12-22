use std::collections::HashMap;

use leptos::*;
use uuid::Uuid;

use crate::{data::Workout, view::head::Head};

#[component]
pub fn WorkoutsForRoutine(workouts: HashMap<Uuid, Workout>, routine_id: Uuid) -> impl IntoView {
    view! {
    <Head></Head>
    <div id="content">
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
                    // a function that returns the items we're iterating over; a signal is fine
                    each=move || workouts.clone()
                    // a unique key for each item
                    key=|(id, _)| id.clone()
                    // renders each item to a view
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
pub fn Workouts(workouts: HashMap<Uuid, Workout>, routine_id: Option<Uuid>) -> impl IntoView {
    view! {
    <Head></Head>
    <div id="content">
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
                    // a function that returns the items we're iterating over; a signal is fine
                    each=move || workouts.clone()
                    // a unique key for each item
                    key=|(id, _)| id.clone()
                    // renders each item to a view
                    children=move |(id, workout): (Uuid, Workout)| {
                        let id_str = id.to_string();
                        view! {
                            <tr>
                            <td>{id_str.clone()}</td>
                            <td>{workout.name}</td>
                            <td>{workout.description}</td>
                            <td>
                                <button class="btn" hx-get={format!("/workouts/{id}")} hx-target="#content" hx-swap="outerHTML" hx-push-url="true">
                                View
                                </button>
                            </td>
                            <td>
                                <DeleteWorkoutFromWorkoutsButton workout_id=id routine_id=routine_id></DeleteWorkoutFromWorkoutsButton>
                            </td>
                            </tr>
                        }
                    }
                />
                <tr>
                <td>
                    <AddWorkoutFromWorkoutsButton routine_id=routine_id></AddWorkoutFromWorkoutsButton>
                </td>
                </tr>
            </tbody>
        </table>
    </div>
    }
}

struct AddButtonVariationData {
    button_text: String,
    add_api_endpoint: String,
}

#[component]
fn AddWorkoutFromWorkoutsButton(routine_id: Option<Uuid>) -> impl IntoView {
    let mut addButtonVariationData = AddButtonVariationData {
        button_text: "Add Workout".to_owned(),
        add_api_endpoint: format!("/workouts"),
    };

    if let Some(routine_id) = routine_id {
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

struct DeleteButtonVariationData {
    button_text: String,
    delete_api_endpoint: String,
}

#[component]
fn DeleteWorkoutFromWorkoutsButton(workout_id: Uuid, routine_id: Option<Uuid>) -> impl IntoView {
    let mut deleteButtonVariationData = DeleteButtonVariationData {
        button_text: "Delete Workout".to_owned(),
        delete_api_endpoint: format!("/workouts/{workout_id}"),
    };

    if let Some(routine_id) = routine_id {
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
