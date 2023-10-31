use actix_web::{HttpMessage, HttpRequest};
use tera::Context;

use super::auth::Claims;

pub struct Extensions;

impl Extensions {
    pub fn unwrap_claims<T: Claims + Clone + 'static>(req: &HttpRequest) -> T {
        let ext = req.extensions();
        ext.get::<T>().cloned().unwrap()
    }

    pub fn unwrap_context(req: &HttpRequest) -> Context {
        let ext = req.extensions();
        ext.get::<Context>().cloned().unwrap_or(Context::new())
    }

    pub fn unwrap_claims_and_context<T: Claims + Clone + 'static>(
        req: &HttpRequest,
    ) -> (T, Context) {
        let ext = req.extensions();

        let claims = ext.get::<T>().cloned().unwrap();
        let context = ext
            .get::<Context>()
            .cloned()
            .unwrap_or(Context::new())
            .to_owned();
        (claims, context)
    }
}
