use leptos::*;

use crate::data::Workout;

#[component]
pub fn ViewGlobalWorkoutsListToAddToRoutine(
    workouts: Vec<Workout>,
    routine_id: i64,
) -> impl IntoView {
    let workout_row_add_button = move |workout_id: i64| {
        view! {
            <td>
                <AddWorkoutToRoutineWorkoutsListButton workout_id=workout_id routine_id=routine_id></AddWorkoutToRoutineWorkoutsListButton>
            </td>
        }.into_view()
    };
    let get_workout_row_component = move |workout: Workout| {
        view! {
            <ViewWorkoutsListRow workout=workout maybe_workout_row_add_button=Some(Box::new(workout_row_add_button)) maybe_workout_row_delete_button=None></ViewWorkoutsListRow>
        }
    };
    let get_workouts_add_button_component = move || {
        view! {
            <ButtonToNavigateToAddWorkoutFormForGlobablWorkouts></ButtonToNavigateToAddWorkoutFormForGlobablWorkouts>
        }
    };
    let get_workouts_back_button_component = move || {
        view! {
            <td>
                <ButtonToNavigateBackFromViewingWorkouts resource_path=format!("/routines/{routine_id}")></ButtonToNavigateBackFromViewingWorkouts>
            </td>
        }.into_view()
    };
    view! {
        <ViewWorkoutsList
            workouts=workouts
            get_workout_row_component=Box::new(get_workout_row_component)
            get_workouts_add_button_component=Box::new(get_workouts_add_button_component)
            div_id="content".to_owned()
            get_workouts_back_button_component=Box::new(get_workouts_back_button_component)>
        </ViewWorkoutsList>
    }
}

#[component]
pub fn ViewGlobalWorkoutsList(workouts: Vec<Workout>) -> impl IntoView {
    let workout_row_delete_button = move |workout_id: i64| {
        view! {
            <td>
                <DeleteWorkoutFromGlobalWorkoutsListButton workout_id=workout_id></DeleteWorkoutFromGlobalWorkoutsListButton>
            </td>
        }.into_view()
    };
    let get_workout_row_component = move |workout: Workout| {
        view! {
            <ViewWorkoutsListRow workout=workout maybe_workout_row_add_button=None maybe_workout_row_delete_button=Some(Box::new(workout_row_delete_button))></ViewWorkoutsListRow>
        }
    };
    let get_workouts_add_button_component = move || {
        view! {
            <ButtonToNavigateToAddWorkoutFormForGlobablWorkouts></ButtonToNavigateToAddWorkoutFormForGlobablWorkouts>
        }
    };
    let get_workouts_back_button_component = move || {
        view! {
            <td>
                <ButtonToNavigateBackFromViewingWorkouts resource_path="/".to_owned()></ButtonToNavigateBackFromViewingWorkouts>
            </td>
        }.into_view()
    };
    view! {
        <ViewWorkoutsList
            workouts=workouts
            get_workout_row_component=Box::new(get_workout_row_component)
            get_workouts_add_button_component=Box::new(get_workouts_add_button_component)
            div_id="content".to_owned()
            get_workouts_back_button_component=Box::new(get_workouts_back_button_component)>
        </ViewWorkoutsList>
    }
}

#[component]
pub fn ViewWorkoutsList(
    workouts: Vec<Workout>,
    get_workout_row_component: Box<dyn Fn(Workout) -> View>,
    get_workouts_add_button_component: Box<dyn Fn() -> View>,
    get_workouts_back_button_component: Box<dyn Fn() -> View>,
    div_id: String,
) -> impl IntoView {
    view! {
        <div id={div_id}>
            <p>Workouts:</p>
            <table>
                <thead>
                    <tr>
                    <th>ID</th>
                    <th>Name</th>
                    <th>Description</th>
                    <th></th>
                    <th></th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || workouts.clone()
                        key=|workout| workout.id.clone()
                        children=move |workout: Workout| {
                            get_workout_row_component(workout)
                        }
                    />
                    <tr>
                    <td>
                        { get_workouts_add_button_component() }
                    </td>
                    { get_workouts_back_button_component() }
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn ViewWorkoutsListRow(
    workout: Workout,
    maybe_workout_row_add_button: Option<Box<dyn Fn(i64) -> View>>,
    maybe_workout_row_delete_button: Option<Box<dyn Fn(i64) -> View>>,
) -> impl IntoView {
    let id = workout.id.to_string();
    view! {
        <tr>
        <td>{id.clone()}</td>
        <td>{workout.name}</td>
        <td>{workout.description}</td>
        <td>
            <button class="btn" hx-get={format!("/workouts/{id}")} hx-target="#content" hx-swap="outerHTML" hx-push-url="true">
            View
            </button>
        </td>
        {
            maybe_workout_row_add_button.map(|workout_row_add_button| workout_row_add_button(workout.id))
        }
        {
            maybe_workout_row_delete_button.map(|workout_row_delete_button| workout_row_delete_button(workout.id))
        }
        </tr>
    }
}

#[component]
pub fn ButtonToNavigateToAddWorkoutFormForRoutineWorkouts(routine_id: i64) -> impl IntoView {
    view! {
        <button class="btn" hx-get={format!("/routines/{routine_id}/workouts/add-workout-form")} hx-push-url="true" hx-target="#content">
            View Workouts to Add To Routine
        </button>
    }
}

#[component]
pub fn ButtonToNavigateToAddWorkoutFormForGlobablWorkouts() -> impl IntoView {
    view! {
        <button class="btn" hx-get="/workouts/add-workout-form" hx-push-url="true" hx-target="#content">
            Add Workout
        </button>
    }
}

#[component]
pub fn AddWorkoutToRoutineWorkoutsListButton(routine_id: i64, workout_id: i64) -> impl IntoView {
    view! {
        <button class="btn" hx-post={format!("/routines/{routine_id}/workouts/{workout_id}", routine_id = routine_id.to_string(), workout_id = workout_id.to_string())} hx-target="closest tr" hx-swap="outerHTML">
        Add Workout to routine
        </button>
    }
}

#[component]
pub fn DeleteWorkoutFromRoutineWorkoutsListButton(
    workout_id: i64,
    routine_id: i64,
) -> impl IntoView {
    view! {
        <button class="btn" hx-delete={format!("/routines/{routine_id}/workouts/{workout_id}")} hx-target="closest tr" hx-swap="outerHTML">
            Delete Workout from Routine
        </button>
    }
}

#[component]
pub fn DeleteWorkoutFromGlobalWorkoutsListButton(workout_id: i64) -> impl IntoView {
    view! {
        <button class="btn" hx-delete={format!("/workouts/{workout_id}")} hx-target="closest tr" hx-swap="outerHTML">
            Delete Workout
        </button>
    }
}

#[component]
pub fn ButtonToNavigateBackFromViewingWorkouts(resource_path: String) -> impl IntoView {
    view! {
        <button class="btn" hx-get={resource_path} hx-target="#content" hx-push-url="true" hx-swap="outerHTML">
            Back
        </button>
    }
}

#[component]
pub fn CreateWorkoutForm() -> impl IntoView {
    view! {
        <div id="content">
            <form hx-post="/workouts" hx-target="#content" hx-ext="json-enc" hx-swap="outerHTML" hx-push-url="true">
                <input hidden name="id" value={-1} type="text"></input>
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
