use leptos::*;

#[component]
pub fn Head() -> impl IntoView {
    view! {
        <head hx-ext="head-support" hx-head="merge">
            <script src="https://unpkg.com/htmx.org@1.9.6"
            integrity="sha384-FhXw7b6AlE/jyjlZH5iHa/tTe9EpJ1Y55RjcgPbjeWMskSxZt1v9qkxLJWNJaGni"
            crossorigin="anonymous"></script>
            <script src="https://unpkg.com/htmx.org/dist/ext/json-enc.js"></script>
            <script src="https://unpkg.com/htmx.org/dist/ext/head-support.js"></script>
        </head>
    }
}
