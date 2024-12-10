use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::infrastructure::AppState;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Debug, Clone)]
pub struct JwtMiddleware {}

impl JwtMiddleware {
    pub fn new() -> Self {
        JwtMiddleware {}
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService { service }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut secret = "";

        // Access app data
        if let Some(data) = req.app_data::<web::Data<AppState>>() {
            secret = data.authentication_config.secret.as_str();
        }

        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(jwt) = auth_str.strip_prefix("Bearer ") {
                    // Decode and validate JWT...
                    let validation = Validation::default();
                    match decode::<serde_json::Value>(
                        jwt,
                        &DecodingKey::from_secret(secret.as_ref()),
                        &validation,
                    ) {
                        Ok(token_data) => {
                            req.extensions_mut().insert(token_data.claims);
                        }
                        Err(e) => {
                            log::error!("JWT validation failed: {:?}", e);

                            return Box::pin(async move {
                                Err(actix_web::error::ErrorUnauthorized("Invalid JWT"))
                            });
                        }
                    }
                }
            }
        } else {
            return Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized(
                    "Missing Authorization header",
                ))
            });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
