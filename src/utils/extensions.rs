use actix_web::{HttpMessage, HttpRequest};
use tera::Context;

use super::claims::Claims;

pub struct Extensions;

impl Extensions {
    pub fn unwrap_claims<T, K>(req: &HttpRequest) -> T
    where
        T: Claims<K> + Clone + 'static,
    {
        let ext = req.extensions();
        ext.get::<T>().cloned().unwrap()
    }

    pub fn unwrap_context(req: &HttpRequest) -> Context {
        let ext = req.extensions();
        ext.get::<Context>().cloned().unwrap_or(Context::new())
    }

    pub fn unwrap_claims_and_context<T, K>(req: &HttpRequest) -> (T, Context)
    where
        T: Claims<K> + Clone + 'static,
    {
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
