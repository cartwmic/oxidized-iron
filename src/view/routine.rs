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
        <Workouts workouts=routine.workouts.unwrap_or_else(|| HashMap::new()) workouts_from_routine=true></Workouts>
    </div>
    }
}
