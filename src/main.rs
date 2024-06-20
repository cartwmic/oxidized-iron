use axum::Router;
use oxidized_iron::{
    domain::{
        exercise::ExerciseService, lifting_log_entry::LiftingLogService, routine::RoutineService,
        workout::WorkoutService, DomainService,
    },
    driven::postgres_repo::PostgresRepository,
    driving::htmx_handler::{get_router, HtmxState},
};
use sqlx::postgres::PgPoolOptions;

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/oxidized-iron")
        .await?;

    let database_repository = Arc::new(PostgresRepository {
        database_connection_pool: Box::new(database_connection_pool),
    });

    let domain_service = DomainService {
        lifting_log_entry_service: LiftingLogService {
            database_repository: database_repository.clone(),
        },
        exercise_service: ExerciseService {
            database_repository: database_repository.clone(),
        },
        routine_service: RoutineService {
            database_repository: database_repository.clone(),
        },
        workout_service: WorkoutService {
            database_repository: database_repository.clone(),
        },
    };

    let my_state = Arc::new(Mutex::new(HtmxState { domain_service }));

    let app = Router::new().merge(get_router()).with_state(my_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
