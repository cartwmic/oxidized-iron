use std::collections::HashMap;

use leptos::*;

use crate::{
    data::Routine,
    view::{head::Head, workouts::Workouts},
};

#[component]
pub fn Routine(routine: Routine) -> impl IntoView {
    view! {
    <Head></Head>
    <div id="content">
        <h2>Routine: { routine.name }</h2>
        <p>id: { routine.id.to_string() }</p>
        <p>description: { routine.description }</p>
        <h3>Workouts</h3>
        <Workouts workouts=routine.workouts.unwrap_or_else(|| HashMap::new())></Workouts>
    </div>
    }
}
