use leptos::*;
use uuid::Uuid;

use crate::view::head::Head;

#[component]
pub fn CreateWorkout() -> impl IntoView {
    view! {
        <Head></Head>
        <div id="content">
            <form hx-post="/workouts" hx-target="#content" hx-ext="json-enc" hx-swap="outerHTML">
                <input hidden name="id" value={Uuid::new_v4().to_string()} type="text"></input>
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
