use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};

use crate::{application::usecases::guild_commanders::GuildCommandersUseCase, domain::{repositories::guild_commanders::GuildCommandersRepository, value_object::adventurer_model::RegisterAdventurerModel}, infrastructure::postgres::{postgres_connection::PgPoolSquad, repositories::guild_commanders::GuildCommanderPostgres}};

pub fn routes(dbpool: Arc<PgPoolSquad>) -> Router {
    let guild_commanders_repository = GuildCommanderPostgres::new(Arc::clone(&dbpool));
    let guild_commanders_use_case = GuildCommandersUseCase::new(Arc::new(guild_commanders_repository));
    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(guild_commanders_use_case))
}

pub async fn register<T>(
    State(guild_commanders_use_case): State<Arc<GuildCommandersUseCase<T>>>,
    Json(register_adventurer_model): Json<RegisterAdventurerModel>,
) -> impl IntoResponse
where
    T: GuildCommandersRepository + Send + Sync,
{
    unimplemented!()
}