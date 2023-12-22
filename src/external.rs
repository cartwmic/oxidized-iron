use std::collections::HashMap;

use uuid::Uuid;

use crate::data::{Routine, Workout};

pub struct MyState {
    pub routines: HashMap<Uuid, Routine>,
    pub workouts: HashMap<Uuid, Workout>,
}
