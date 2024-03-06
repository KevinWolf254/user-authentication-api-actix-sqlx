use std::future::{ready, Ready};

use actix_web::{HttpRequest, error::{ErrorUnauthorized, ErrorInternalServerError, ErrorBadRequest}, http, web, dev::Payload, Error as ActixWebError, FromRequest};
use crate::{jwt, AppState};
use log::error;

pub struct JwtAuthenticationGuard {
    pub id: i32
}

impl FromRequest for JwtAuthenticationGuard {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get(http::header::AUTHORIZATION);
        let app_data = match req.app_data::<web::Data<AppState>>() {
            Some(data) => data,
            None => return ready(Err(ErrorInternalServerError("Failed to retrieve app state").into()))
        };

        match auth_header {
            Some(token) => {
                if let Ok(token_str) = token.to_str() {
                    if token_str.is_empty() || token_str.len() < 8{
                        return ready(Err(ErrorUnauthorized("Authorization is required!").into()));
                    }
                    let jwt = &token_str[7..];
                    let result = jwt::validate_token(jwt, &app_data.jwt_config)
                        .map(|claims| JwtAuthenticationGuard { id: claims.user.user_id })
                        .map_err(|error| {
                            error!("{}", error);
                            ErrorUnauthorized("Authorization is required!").into()
                        });

                    ready(result)
                } else {
                    ready(Err(ErrorBadRequest("Invalid token format").into()))
                }
            },
            None => ready(Err(ErrorUnauthorized("Authorization is required!").into()))
        }
    }
}