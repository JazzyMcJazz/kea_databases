use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    server::AppState,
    utils::{auth::DdbClaims, extensions::Extensions},
};

// GET /relania
pub async fn index(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let tera = &data.tera;
    let _db = &data.surreal;

    let (_claims, mut context) = Extensions::unwrap_claims_and_context::<DdbClaims>(&req);

    let characters: Vec<String> = vec![]; // TODO: Get characters from database
    context.insert("characters", &characters);

    let Ok(html) = tera.render("documenia/index.html", &context) else {
        return HttpResponse::InternalServerError().body("Template error");
    };

    HttpResponse::Ok().body(html)
}
