use std::sync::Arc;

use axum::Router;

use crate::{application::usecases::authentication::AuthenticationUseCase, infrastructure::postgres::{postgres_connection::PgPoolSquad, repositories::{adventures::AdventurerPostgres, guild_commanders::GuildCommanderPostgres}}};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurers_repository = AdventurerPostgres::new(Arc::clone(&db_pool));
    let guild_commanders_repository = GuildCommanderPostgres::new(Arc::clone(&db_pool));
    let adventurers_use_case = AuthenticationUseCase::new(
        Arc::new(adventurers_repository),
        Arc::new(guild_commanders_repository)
    );

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(adventurers_use_case))
}

