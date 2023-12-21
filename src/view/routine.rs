use leptos::*;

use crate::{data::Routine, view::head::Head};

#[component]
pub fn Routine(routine: Routine) -> impl IntoView {
    view! {
    <Head></Head>
    <div id="content">
        <h2>{ routine.name }</h2>
        <p>id: { routine.id.to_string() }</p>
        <p>description: { routine.description }</p>
    </div>
    }
}
