use actix_web::{post, web, HttpResponse, Responder};

use super::{db::Games, model::GameConfig};

/// Registers all authentication endpoints.
pub(super) fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/a/games").service(create));
}

#[post("/")]
async fn create(games: web::Data<Games>, game_config: web::Json<GameConfig>) -> impl Responder {
    // TODO
    HttpResponse::Ok().finish()
}
