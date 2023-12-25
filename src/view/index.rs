use leptos::*;

use crate::view::head::Head;

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <html lang="en">
            <Head></Head>
            <body>
                <div id="content">
                    <button hx-get="/routines" hx-target="#content" hx-push-url="true" hx-swap="outerHTML">View Routines</button>
                    <button hx-get="/workouts" hx-target="#content" hx-push-url="true" hx-swap="outerHTML">View Workouts</button>
                </div>
            </body>
        </html>
    }
}
