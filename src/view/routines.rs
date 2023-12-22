use std::collections::HashMap;

use leptos::*;
use uuid::Uuid;

use crate::{data::Routine, view::head::Head};

#[component]
pub fn Routines(routines: HashMap<Uuid, Routine>) -> impl IntoView {
    view! {
    <Head></Head>
    <div id="content">
        <p>Routines:</p>
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
                    each=move || routines.clone()
                    key=|(id, _)| id.clone()
                    children=move |(id, routine): (Uuid, Routine)| {
                        let id = id.to_string();
                        view! {
                            <tr>
                            <td>{id.clone()}</td>
                            <td>{routine.name}</td>
                            <td>{routine.description}</td>
                            <td>
                                <button class="btn" hx-get={format!("/routines/{id}")} hx-target="#content" hx-swap="outerHTML" hx-push-url="true">
                                View
                                </button>
                            </td>
                            <td>
                                <button class="btn" hx-delete={format!("/routines/{id}")} hx-target="closest tr" hx-swap="outerHTML">
                                Delete
                                </button>
                            </td>
                            </tr>
                        }
                    }
                />
                <tr>
                <td>
                    <button class="btn" hx-post="/routines" hx-target="#content">
                    Add
                    </button>
                </td>
                </tr>
            </tbody>
        </table>
    </div>
    }
}
