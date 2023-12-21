use std::collections::HashMap;

use uuid::Uuid;

use crate::data::Routine;

pub struct MyState {
    pub routines: HashMap<Uuid, Routine>,
}
