use std::collections::HashMap;

use leptos::*;
use uuid::Uuid;

use crate::{data::Workout, view::head::Head};

#[component]
pub fn Workouts(workouts: HashMap<Uuid, Workout>, workouts_from_routine: bool) -> impl IntoView {
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
                                <DeleteWorkoutFromWorkoutsButton id=id workouts_from_routine=workouts_from_routine></DeleteWorkoutFromWorkoutsButton>
                            </td>
                            </tr>
                        }
                    }
                />
                <tr>
                <td>
                    <button class="btn" hx-post="/workouts" hx-target="#content">
                    Add
                    </button>
                </td>
                </tr>
            </tbody>
        </table>
    </div>
    }
}

#[component]
pub fn DeleteWorkoutFromWorkoutsButton(id: Uuid, workouts_from_routine: bool) -> impl IntoView {
    let button_text = if workouts_from_routine {
        "Delete From Routine"
    } else {
        "Delete Workout"
    };

    view! {
        <button class="btn" hx-delete={format!("/workouts/{id}")} hx-target="closest tr" hx-swap="outerHTML">
            { button_text }
        </button>
    }
}
