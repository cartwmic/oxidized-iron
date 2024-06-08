use std::hash::Hash;

use leptos::*;

use crate::{
    data::{Exercise, LiftingLogEntry},
    GetTableData, GetUrlPrefix, TableData,
};

pub fn base(content: View) -> impl IntoView {
    view! {
        <html lang="en">
            <head hx-ext="head-support" hx-head="merge">
                <script src="https://unpkg.com/htmx.org@1.9.10" crossorigin="anonymous"></script>
                <script src="https://unpkg.com/htmx.org/dist/ext/json-enc.js"></script>
                <script src="https://unpkg.com/htmx.org/dist/ext/head-support.js"></script>
            </head>
            <body>
                <div id={ crate::BASE_CONTENT_DIV_ID }>
                    { content }
                </div>
            </body>
        </html>
    }
}

#[component]
pub fn ViewLiftingLogButton() -> impl IntoView {
    view! {
        <button hx-get="/lifting_log" hx-push-url="true" hx-target={ crate::format_id_to_htmx_target_(crate::BASE_CONTENT_DIV_ID.to_string()) }>View Lifting Log</button>
    }
}

#[component]
pub fn ViewLiftingLog(lifting_log: Vec<LiftingLogEntry>) -> impl IntoView {
    let table_data = TableData::new(lifting_log);
    base_table(
        table_data,
        "lifting-log-table".to_string(),
        "Lifting Log".to_string(),
    )
}

#[component]
pub fn ViewExercisesButton() -> impl IntoView {
    view! {
        <button hx-get="/exercises" hx-push-url="true" hx-target={ crate::format_id_to_htmx_target_(crate::BASE_CONTENT_DIV_ID.to_string()) }>View Exercises</button>
    }
}

#[component]
pub fn ViewExercises(exercises: Vec<Exercise>) -> impl IntoView {
    let table_data = TableData::new(exercises);
    base_table(
        table_data,
        "exercise-table".to_string(),
        "Exercises".to_string(),
    )
}

pub fn base_table<T: GetTableData + GetUrlPrefix + Clone + Hash + Eq + 'static>(
    table_data: TableData<T>,
    id: String,
    title: String,
) -> impl IntoView {
    view! {
         <div id=id>
            <p>{ title }</p>
            <table>
                <thead>
                    <tr>
                    <For
                        each=move || table_data.headers.clone()
                        key=|header| header.clone()
                        children=move |header: String| {
                            view! {
                                <th>{ header }</th>
                            }
                        }
                    />
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || table_data.records.clone()
                        key=|record| record.get_data_id()
                        children=move |record: T| {
                            let id = record.get_data_id();
                            let url_prefix = record.get_url_prefix();
                            view! {
                                <tr>
                                    <For
                                        each=move || record.get_table_data().clone()
                                        key=|datum| datum.clone()
                                        children=move |datum: String| {
                                            view! {
                                                <td>{ datum }</td>
                                            }
                                        }
                                    />
                                    <td>
                                        <button class="btn" hx-get={format!("/{url_prefix}/{id}")} hx-push-url="true">
                                            View
                                        </button>
                                    </td>
                                    <td>
                                        <button class="btn" hx-get="/{url_prefix}/add" hx-push-url="true">
                                            Add
                                        </button>
                                    </td>
                                    <td>
                                        <button class="btn" hx-delete={format!("/{url_prefix}/{id}")} hx-target="closest tr" hx-swap="outerHTML">
                                            Delete
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
