use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use crate::lib::ansi::graphics;
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::time::Instant;

pub struct RequestInfoFactory;

impl RequestInfoFactory {
    pub fn new() -> Self {
        RequestInfoFactory {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestInfoFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestInfoMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestInfoMiddleware { service }))
    }
}

pub struct RequestInfoMiddleware<S> {
    service: S,
}

impl<S> RequestInfoMiddleware<S> {
    pub fn log_request_info(&self, req: &ServiceRequest) {
        let conn_info = req.connection_info();
        let ip = conn_info.realip_remote_addr()
            .and_then(|ip| Some(format!("{}={}", graphics::bold("ip"), ip)))
            .or(Some("".to_owned()))
            .unwrap();

        let host = format!("{}={}", graphics::bold("host"), conn_info.host());
        let method = format!("{}={}", graphics::bold("method"), req.method());
        let path = format!("{}={}", graphics::bold("path"), req.path());

        log::info!("{}", format!("{method} {path} {host} {ip}"));
    }
}

impl<S, B> Service<ServiceRequest> for RequestInfoMiddleware<S>
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
        let duration = Instant::now();

        self.log_request_info(&req);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            log::info!("duration={}", duration.elapsed().as_millis());
            Ok(res)
        })
    }
}
