use leptos::*;

use crate::data::LiftingLogEntry;

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
pub fn ViewLiftingLogButton() -> impl IntoView {
    view! {
        <button hx-get="/lifting_log" hx-push-url="true">View Lifting Log</button>
    }
}

#[component]
pub fn ViewLiftingLog(lifting_log: Vec<LiftingLogEntry>) -> impl IntoView {
    view! {
         <div id="global-lifting-log">
            <p>lifting_log:</p>
            <table>
                <thead>
                    <tr>
                    <th>ID</th>
                    <th>Rep Count</th>
                    <th>Set Kind</th>
                    <th>RPE</th>
                    <th>Exercise</th>
                    <th>Workout</th>
                    <th>Routine</th>
                    <th>Created At</th>
                    <th></th>
                    <th></th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || lifting_log.clone()
                        key=|lifting_log_entry| lifting_log_entry.id.clone()
                        children=move |lifting_log_entry: LiftingLogEntry| {
                            view! {
                                <tr>
                                    <td>{lifting_log_entry.id.clone()}</td>
                                    <td>{lifting_log_entry.rep_count}</td>
                                    <td>{lifting_log_entry.set_kind}</td>
                                    <td>{lifting_log_entry.rpe}</td>
                                    <td>{lifting_log_entry.exercise}</td>
                                    <td>{lifting_log_entry.workout}</td>
                                    <td>{lifting_log_entry.routine}</td>
                                    <td>{lifting_log_entry.created_at.to_string()}</td>
                                    <td>
                                        <button class="btn" hx-get={let lifting_log_entry_id = lifting_log_entry.id; format!("/lifting_log/{lifting_log_entry_id}")} hx-target="#content" hx-swap="outerHTML" hx-push-url="true">
                                            View
                                        </button>
                                    </td>
                                    <td>
                                        <button class="btn" hx-get="/lifting_log/add-lifting_log_entry-form" hx-push-url="true" hx-target="#content">
                                            Add lifting_log_entry
                                        </button>
                                    </td>
                                    <td>
                                        <button class="btn" hx-delete={let lifting_log_entry_id = lifting_log_entry.id; format!("/lifting_log/{lifting_log_entry_id}")} hx-target="closest tr" hx-swap="outerHTML">
                                            Delete lifting_log_entry
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
