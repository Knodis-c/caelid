use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use crate::internal::ansi::graphics;
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    time::Instant
};
use uuid::Uuid;

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
    service: S
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
        let timestamp = Instant::now();

        let output = {
            let request_id = Uuid::new_v4();
            let conn_info = req.connection_info();
            let ip = conn_info.realip_remote_addr()
                .and_then(|ip| Some(format!("{}={}", graphics::bold("ip"), ip)))
                .unwrap_or("".to_owned());
            let resource = req
                .resource_map()
                .match_name(req.path())
                .map(|path| format!("{}={}", graphics::bold("resource"), path))
                .unwrap_or("".to_owned());

            let host = format!("{}={}", graphics::bold("host"), conn_info.host());
            let method = format!("{}={}", graphics::bold("method"), req.method());
            let path = format!("{}={}", graphics::bold("path"), req.path());
            let req_id = format!("{}={}", graphics::bold("request_id"), request_id);

            format!("{method} {path} {host} {req_id} {ip} {resource}")
        };

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let status = res.status().as_u16();

            log::info!(
                "{output} {}={}ms {}={}",
                graphics::bold("duration"),
                timestamp.elapsed().as_millis(),
                graphics::bold("status"),
                status
            );

            Ok(res)
        })
    }
}
