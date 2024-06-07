use std::hash::Hash;

use leptos::*;

use crate::{data::LiftingLogEntry, GetTableData, GetUrlPrefix, TableData};

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
    let headers = vec![
        "id".to_string(),
        "rep_count".to_string(),
        "set_kind".to_string(),
        "rpe".to_string(),
        "exercise".to_string(),
        "workout".to_string(),
        "routine".to_string(),
        "created_at".to_string(),
    ];
    let table_data = TableData::new(headers, lifting_log);
    base_table(
        table_data,
        "lifting-log-table".to_string(),
        "Lifting Log".to_string(),
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
