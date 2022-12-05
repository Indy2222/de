use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    web, App, FromRequest, HttpMessage,
};
use actix_web_httpauth::{
    extractors::{basic::BasicAuth, bearer::BearerAuth},
    middleware::HttpAuthentication,
};
use log::warn;

use super::token::Tokens;

// TODO HttpAuthentication::bearer(validator)

async fn validator(
    mut req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let tokens = match req.extract::<web::Data<Tokens>>().await {
        Ok(tokens) => tokens,
        Err(error) => return Err((error, req)),
    };

    let claims = match tokens.decode(credentials.token()) {
        Ok(claims) => claims,
        Err(error) => {
            warn!("Token decoding error: {:?}", error);
            return Err((
                actix_web::error::ErrorUnauthorized("Invalid JWT provided"),
                req,
            ));
        }
    };

    let previous = req.extensions_mut().insert(claims);
    assert!(previous.is_none());

    Ok(req)
}
