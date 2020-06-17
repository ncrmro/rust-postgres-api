use futures::future::{err, ok, Ready};

use actix_web::dev::Payload;
use actix_web::dev::PayloadStream;

use crate::core::db::model::fields;
use crate::core::http;
use crate::core::http::errors;
use crate::core::http::Apiv2Schema;
use crate::core::http::FromRequest;

pub type ViewerId = Option<fields::Id>;

#[derive(Serialize, Deserialize, Apiv2Schema, Copy, Clone)]
pub struct Viewer {
    pub id: ViewerId,
}

impl FromRequest for Viewer {
    type Error = errors::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(
        req: &http::HttpRequest,
        _payload: &mut Payload<PayloadStream>,
    ) -> Self::Future {
        let ext = req.extensions();
        let viewer = ext.get::<Viewer>();
        if let Some(viewer) = viewer {
            ok(*viewer)
        } else {
            err(errors::ErrorBadRequest(
                "This password or username is incorrect.",
            ))
        }
    }
}
