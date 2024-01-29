use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Routine {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub workouts: Option<Vec<Workout>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Workout {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// This is the `#[derive(Type)]` macro expanded for the `Workout` struct above. I have to do this
// manually (and remove `String: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,`) from the trait
// bounds due to https://github.com/launchbadge/sqlx/issues/1031#issuecomment-774001746
impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for Workout
where
    i64: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    i64: ::sqlx::types::Type<::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<String>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<String>: ::sqlx::types::Type<::sqlx::Postgres>,
    chrono::DateTime<chrono::Utc>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    chrono::DateTime<chrono::Utc>: ::sqlx::types::Type<::sqlx::Postgres>,
    chrono::DateTime<chrono::Utc>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    chrono::DateTime<chrono::Utc>: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn decode(
        value: ::sqlx::postgres::PgValueRef<'r>,
    ) -> ::std::result::Result<
        Self,
        ::std::boxed::Box<
            dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
        >,
    > {
        let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i64>()?;
        let name = decoder.try_decode::<String>()?;
        let description = decoder.try_decode::<Option<String>>()?;
        let created_at = decoder.try_decode::<chrono::DateTime<chrono::Utc>>()?;
        let updated_at = decoder.try_decode::<chrono::DateTime<chrono::Utc>>()?;
        ::std::result::Result::Ok(Workout {
            id,
            name,
            description,
            created_at,
            updated_at,
        })
    }
}
#[automatically_derived]
impl ::sqlx::Type<::sqlx::Postgres> for Workout {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("Workout")
    }
}
