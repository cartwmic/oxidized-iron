use leptos::*;
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

#[derive(Serialize, Deserialize, Clone)]
pub struct Rep {
    pub id: i64,
    pub kind: RepKind,
    pub set: i64,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Clone, sqlx::Type, AsRefStr)]
#[sqlx(type_name = "rep_kind")]
pub enum RepKind {
    Normal,
    Myo,
    LengthenedPartial,
    Drop,
}

impl IntoView for RepKind {
    fn into_view(self) -> leptos::View {
        view! {
            {self.as_ref().to_string()}
        }
        .into_view()
    }
}

// for nested objects, should try https://www.reddit.com/r/rust/comments/1bm8vep/comment/kwdb9tj/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
//
// goal now is to do a Bill of Materials schema design where we have one table that does many self
// joins. Could do something like a single table called "lifting_datum" with id, kind,
// descripiton/notes, parent_datum, etc. This would hold hierarchy of routines <<->> excercies
// ->> sets ->> reps but only include columns relevant to the actual hierarchy. For additiona
// attributes (like rep kind, set rpe, excercise name, etc., create bespoke tables for those datum
// kinds with this aux info e.g.: set_aux, rep_aux, excercise_aux, etc.)
