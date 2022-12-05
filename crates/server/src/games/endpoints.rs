use actix_web::{post, web, HttpResponse, Responder};

use super::{
    db::Games,
    model::{Game, GameConfig},
};
use crate::auth::Claims;

/// Registers all authentication endpoints.
pub(super) fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/a/games").service(create));
}

#[post("/")]
async fn create(
    games: web::Data<Games>,
    claims: web::ReqData<Claims>,
    game_config: web::Json<GameConfig>,
) -> impl Responder {
    let game = Game::new(game_config.into_inner(), claims.username().to_owned());
    // TODO handle result
    games.create(game).await.unwrap();

    HttpResponse::Ok().finish()
}
