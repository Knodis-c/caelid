use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use crate::internal::ansi::colors;
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    time::Instant
};
use uuid::Uuid;

/// For logging salient request information.
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
            let method = Some(format!("{}={}", colors::bold("method"), req.method()));

            let path = Some(format!("{}={}", colors::bold("path"), req.path()));

            let resource = req
                .resource_map()
                .match_name(req.path())
                .map(|path| format!("{}={}", colors::bold("resource"), path));

            let host = Some(format!("{}={}", colors::bold("host"), req.connection_info().host()));

            let ip = req.connection_info().realip_remote_addr()
                .map(|ip| format!("{}={}", colors::bold("ip"), ip));

            let req_id = Some(format!("{}={}", colors::bold("request_id"), Uuid::new_v4()));

            let mut log = vec![];

            method.and_then(|i| Some(log.push(i.to_owned())));
            path.and_then(|i| Some(log.push(i.to_owned())));
            resource.and_then(|i| Some(log.push(i.to_owned())));
            host.and_then(|i| Some(log.push(i.to_owned())));
            ip.and_then(|i| Some(log.push(i.to_owned())));
            req_id.and_then(|i| Some(log.push(i.to_owned())));

            log.join(" ")
        };

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let status = res.status().as_u16();

            //log::info!(
                //"{output} {}={}ms {}={}",
                //colors::bold("duration"),
                //timestamp.elapsed().as_millis(),
                //colors::bold("status"),
                //status
            //);

            Ok(res)
        })
    }
}
