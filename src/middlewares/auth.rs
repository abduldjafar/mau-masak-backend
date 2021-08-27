use actix_web::{Error, HttpResponse, dev::{ServiceRequest, ServiceResponse}, http::{Method, HeaderName, HeaderValue}, web::Data, HttpMessage, HttpRequest};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use actix_web::dev::{Transform, Service};
use crate::services::users::UserAppState;
use crate::constants;
use crate::entity::responses::ResponseBody;

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}
pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = false;


        // Bypass some account routes
        let headers = req.headers_mut();
        headers.append(HeaderName::from_static("content-length"),HeaderValue::from_static("true"));
        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            for ignore_route in constants::IGNORE_ROUTES.iter() {
                if req.path().starts_with(ignore_route) {
                    authenticate_pass = true;
                    break;
                }
            }
            if !authenticate_pass {
                if let Some(userAppState) = req.app_data::<Data<UserAppState>>() {
                    println!("Connecting to database...");
                    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
                        println!("Parsing authorization header...");
                        if let Ok(authen_str) = authen_header.to_str() {
                            if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                                println!("Parsing token...");
                                let token = authen_str[6..authen_str.len()].trim();
                                let decode_token = crate::entity::jwt::UserToken::decode_token(token.to_string());

                                if decode_token.is_ok(){
                                    authenticate_pass = true;
                                }
                            }else {
                                authenticate_pass = false;
                            }
                        }
                    }
                }
            }
        }
        if authenticate_pass {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(ResponseBody::new(
                            constants::MESSAGE_INVALID_TOKEN,
                            401,
                            true,
                            constants::EMPTY,
                        ))
                        .into_body(),
                ))
            })
        }
    }
}
