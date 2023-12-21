use std::collections::HashMap;

use leptos::*;
use uuid::Uuid;

use crate::{data::Workout, view::head::Head};

#[component]
pub fn Workouts(workouts: HashMap<Uuid, Workout>) -> impl IntoView {
    view! {
    <Head></Head>
    <div id="content">
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
                        let id = id.to_string();
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
                                <button class="btn" hx-delete={format!("/workouts/{id}")} hx-target="closest tr" hx-swap="outerHTML">
                                Delete from routine
                                </button>
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
