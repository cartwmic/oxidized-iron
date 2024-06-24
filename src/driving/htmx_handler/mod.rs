use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, post},
    Router,
};
use exercise::{get_exercises, ViewExercisesButton};
use leptos::*;
use lifting_log_entry::{
    add_lifting_log_entry, delete_lifting_log_entry, get_add_lifting_log_row, get_lifting_log,
    ViewLiftingLogButton,
};
use routine::{get_routines, ViewRoutinesButton};
use ssr::render_to_string;
use tokio::sync::Mutex;
use workout::{get_workouts, ViewWorkoutsButton};

use crate::domain::DomainService;

mod exercise;
mod lifting_log_entry;
mod routine;
mod workout;

const BASE_CONTENT_DIV_ID: &str = "base-content";

pub fn format_id_to_htmx_target_(id: String) -> String {
    format!("#{id}")
}

pub trait GetTableData {
    fn get_table_data(&self) -> Vec<String>;

    fn get_data_id(&self) -> String;

    fn get_headers() -> Vec<TableDataHeader>;
}

pub struct HtmxState {
    pub domain_service: DomainService,
}

pub trait GetUrlPrefix {
    fn get_url_prefix() -> String;
}

#[derive(Clone)]
pub struct TableDataHeader {
    header: String,
    value_is_editable: bool,
}

pub struct TableData<T>
where
    T: GetTableData,
{
    headers: Vec<TableDataHeader>,
    records: Vec<T>,
}

impl<T> TableData<T>
where
    T: GetTableData,
{
    pub fn new(records: Vec<T>) -> TableData<T> {
        let headers = T::get_headers();
        if records.is_empty() {
            TableData { headers, records }
        } else {
            if headers.len() != records.first().unwrap().get_table_data().len() {
                panic!();
            } else {
                TableData { headers, records }
            }
        }
    }

    pub fn get_headers(&self) -> &Vec<TableDataHeader> {
        self.headers.as_ref()
    }

    pub fn get_records(&self) -> &Vec<T> {
        self.records.as_ref()
    }
}

pub fn get_router() -> Router<Arc<Mutex<HtmxState>>> {
    Router::new()
        .route("/", get(index))
        .route("/lifting-log", post(add_lifting_log_entry))
        .route("/lifting-log", get(get_lifting_log))
        .route(
            "/lifting-log/:lifting_log_id",
            delete(delete_lifting_log_entry),
        )
        .route("/lifting-log/add", get(get_add_lifting_log_row))
        .route("/exercises", get(get_exercises))
        .route("/workouts", get(get_workouts))
        .route("/routines", get(get_routines))
}

pub fn base(content: View) -> impl IntoView {
    view! {
        <html lang="en">
            <head hx-ext="head-support" hx-head="merge">
                <script src="https://unpkg.com/htmx.org@1.9.10" crossorigin="anonymous"></script>
                <script src="https://unpkg.com/htmx.org/dist/ext/json-enc.js"></script>
                <script src="https://unpkg.com/htmx.org/dist/ext/head-support.js"></script>
            </head>
            <body>
                <div id={ BASE_CONTENT_DIV_ID }>
                    { content }
                </div>
            </body>
        </html>
    }
}

pub async fn index() -> impl IntoResponse {
    let html = render_to_string(|| {
        base(
            view! {
                <ViewLiftingLogButton></ViewLiftingLogButton>
                <ViewExercisesButton></ViewExercisesButton>
                <ViewWorkoutsButton></ViewWorkoutsButton>
                <ViewRoutinesButton></ViewRoutinesButton>
            }
            .into_view(),
        )
    })
    .to_string();
    (StatusCode::OK, Html(html))
}

pub fn render_table_row<T: GetTableData + GetUrlPrefix + Clone + 'static>(
    record: T,
) -> impl IntoView {
    let id = record.get_data_id();
    let url_prefix = T::get_url_prefix();
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
                <button class="btn" hx-delete={format!("/{url_prefix}/{id}")} hx-target="closest tr" hx-swap="outerHTML">
                    Delete
                </button>
            </td>
        </tr>
    }
}

pub fn base_add_row<T: GetTableData + GetUrlPrefix + Clone + 'static>(
    table_data: TableData<T>,
) -> impl IntoView {
    view! {
        <tr id="new-row">
            <For
                each=move || table_data.headers.clone()
                key=|header| header.header.clone()
                children=move |header| {
                    view! {
                        <td>{
                            if header.value_is_editable {
                                view!{
                                    // must include name so the payload has a key, otherwise htmx
                                    // won't send anything
                                    <input name={header.header} id="new-input-value"></input>
                                }.into_view()
                            } else {
                                view!{
                                    { header.header }
                                }.into_view()
                            }
                        }</td>
                    }
                }
            />
        </tr>
        <tr id="new-row">
            <td>
                <button hx-post="/lifting-log" hx-push-url="true" hx-target="#new-row" hx-swap="outerHTML" hx-include="#new-input-value">Submit</button>
            </td>
        </tr>
    }
}

pub fn base_table<T: GetTableData + GetUrlPrefix + Clone + 'static>(
    table_data: TableData<T>,
    id: String,
    title: String,
) -> impl IntoView {
    let url_prefix = T::get_url_prefix();
    view! {
         <div id=id>
            <p>{ title }</p>
            <table>
                <thead>
                    <tr>
                    <For
                        each=move || table_data.headers.clone()
                        key=|header| header.header.clone()
                        children=move |header| {
                            view! {
                                <th>{ header.header }</th>
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
                            render_table_row(record)
                        }
                    />
                    <tr>
                        <td>
                            <button class="btn" hx-get={format!("/{url_prefix}/add")} hx-target="closest tr" hx-swap="outerHTML">
                                Add
                            </button>
                        </td>

                    </tr>
                </tbody>
            </table>
        </div>
    }
}
