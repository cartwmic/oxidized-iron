use leptos::*;

use crate::data::Rep;

pub fn base(content: View) -> impl IntoView {
    view! {
        <html lang="en">
            <head hx-ext="head-support" hx-head="merge">
                <script src="https://unpkg.com/htmx.org@1.9.10" crossorigin="anonymous"></script>
                <script src="https://unpkg.com/htmx.org/dist/ext/json-enc.js"></script>
                <script src="https://unpkg.com/htmx.org/dist/ext/head-support.js"></script>
            </head>
            <body>
                <div id="content">
                    { content }
                </div>
            </body>
        </html>
    }
}

#[component]
pub fn ViewRepsButton() -> impl IntoView {
    view! {
        <button hx-get="/reps" hx-push-url="true">View Reps</button>
    }
}

#[component]
pub fn ViewGlobalRepsList(reps: Vec<Rep>) -> impl IntoView {
    view! {
         <div id="global-reps-list">
            <p>reps:</p>
            <table>
                <thead>
                    <tr>
                    <th>ID</th>
                    <th>Kind</th>
                    <th>Notes</th>
                    <th></th>
                    <th></th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || reps.clone()
                        key=|rep| rep.id.clone()
                        children=move |rep: Rep| {
                            view! {
                                <tr>
                                    <td>{rep.id.clone()}</td>
                                    <td>{rep.kind}</td>
                                    <td>{rep.notes}</td>
                                    <td>
                                        <button class="btn" hx-get={let rep_id = rep.id; format!("/reps/{rep_id}")} hx-target="#content" hx-swap="outerHTML" hx-push-url="true">
                                        View
                                        </button>
                                    </td>
                                    <td>
                                        <button class="btn" hx-get="/reps/add-rep-form" hx-push-url="true" hx-target="#content">
                                            Add rep
                                        </button>
                                    </td>
                                    <td>
                                        <button class="btn" hx-delete={let rep_id = rep.id; format!("/reps/{rep_id}")} hx-target="closest tr" hx-swap="outerHTML">
                                            Delete rep
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
