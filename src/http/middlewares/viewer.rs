use std::task::{Context, Poll};

// use crate::user::User;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, LocalBoxFuture, Ready};
use futures::FutureExt;
use futures_util::lock::Mutex;
use sqlx::PgPool;
use std::sync::Arc;
// use std::sync::Mutex;
// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Viewer;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Viewer
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ViewerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ViewerMiddleware {
            service: Arc::new(Mutex::new(service)),
        })
    }
}

pub struct ViewerMiddleware<S> {
    service: Arc<Mutex<S>>,
}

impl<S, B> Service for ViewerMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    // type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    type Future = LocalBoxFuture<'static, Result<ServiceResponse<B>, Error>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service
            .try_lock()
            .expect("AuthenticationMiddleware was called already")
            .poll_ready(ctx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        let auth = req.headers().get("Authorization").cloned();
        let conn = req.app_data::<PgPool>().unwrap().get_ref().clone();

        // Note: cloning the mutex, not the service itself
        let inner = self.service.clone();

        async move {
            println!("FKIN ASYNC BITCH");
            // let user = User::verify_token(auth.unwrap().to_str().unwrap().parse().unwrap(), &conn)
            //     .await
            //     .unwrap();
            let mut service = inner.lock().await;

            service.call(req).await
        }
        .boxed_local()
    }
}
