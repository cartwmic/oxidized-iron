use leptos::*;

use crate::{data::Routine, view::routine_workouts::ViewWorkoutsListForRoutine};

#[component]
pub fn ViewRoutinesList(routines: Vec<Routine>) -> impl IntoView {
    view! {
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
                        key=|routine| routine.id.clone()
                        children=move |routine: Routine| {
                            view! {
                                <ViewRoutinesListRow routine=routine></ViewRoutinesListRow>
                            }
                        }
                    />
                    <tr>
                    <td>
                        <button class="btn" hx-get="/routines/add-routine-form" hx-push-url="true" hx-target="#content">
                        Add
                        </button>
                    </td>
                    <td>
                        <button class="btn" hx-get="/" hx-push-url="true" hx-target="#content">
                        Back
                        </button>
                    </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn ViewRoutinesListRow(routine: Routine) -> impl IntoView {
    let id = routine.id.to_string();
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

#[component]
pub fn ViewRoutine(routine: Routine) -> impl IntoView {
    view! {
        <div id="content">
            <h2>Routine: { routine.name }</h2>
            <p>id: { routine.id.to_string() }</p>
            <p>description: { routine.description }</p>
            <ViewWorkoutsListForRoutine workouts=routine.workouts.unwrap_or_else(|| Vec::new()) routine_id=routine.id></ViewWorkoutsListForRoutine>
        </div>
    }
}

#[component]
pub fn CreateRoutineForm() -> impl IntoView {
    view! {
        <div id="content">
            <form hx-post="/routines" hx-target="#content" hx-ext="json-enc" hx-swap="outerHTML" hx-push-url="true">
                <input hidden name="id" value={-1} type="text"></input>
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
