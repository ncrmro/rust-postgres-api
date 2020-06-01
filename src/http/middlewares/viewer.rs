use std::task::{Context, Poll};

use crate::user::User;
use actix_http::Extensions;
use actix_service::{Service, Transform};

use actix_web::{dev::ServiceRequest, dev::ServiceResponse, web::Data, Error, HttpMessage};
use futures::future::{ok, LocalBoxFuture, Ready};
use futures::FutureExt;
use futures_util::lock::Mutex;
use sqlx::PgPool;
use std::rc::Rc;
use std::sync::Arc;

pub struct Viewer;

impl<S, B> Transform<S> for Viewer
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ViewerMiddleware<S>;
    type InitError = ();
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

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        let auth = req.headers().get("Authorization").cloned();
        let conn = req.app_data::<PgPool>().unwrap().get_ref().clone();

        // Note: cloning the mutex, not the service itself
        let inner = self.service.clone();

        async move {
            let mut service = inner.lock().await;
            if auth.is_none() {
                return service.call(req).await;
            }

            let token = auth.unwrap().to_str().unwrap().replace("Bearer ", "");
            match User::verify_token(token, &conn).await {
                Ok(user) => {
                    req.extensions_mut().insert(user);
                }
                _ => {}
            };
            service.call(req).await
        }
        .boxed_local()
    }
}
