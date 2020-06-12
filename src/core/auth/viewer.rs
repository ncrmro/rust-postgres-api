use futures::future::{err, ok, Ready};

use crate::core::db::model::fields;
use crate::core::http::web;
use crate::core::http::Apiv2Schema;
use crate::core::http::Error;
use crate::core::http::ErrorBadRequest;
use crate::core::http::FromRequest;
use crate::core::http::Payload;
use crate::core::http::PayloadStream;

pub type ViewerId = Option<fields::Id>;

#[derive(Serialize, Deserialize, Apiv2Schema, Debug, Copy, Clone)]
pub struct Viewer {
    pub id: ViewerId,
}

impl FromRequest for Viewer {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &web::HttpRequest, _payload: &mut Payload<PayloadStream>) -> Self::Future {
        let ext = req.extensions();
        let viewer = ext.get::<Viewer>();
        if let Some(viewer) = viewer {
            ok(*viewer)
        } else {
            err(ErrorBadRequest("This password or username is incorrect."))
        }
    }
}
