use actix_web::{HttpResponse, HttpRequest, web, HttpMessage};
use tera::Context;

use crate::{server::AppState, utils::auth::RdbClaims, repo::rdbms::character_repo::CharacterRepo};

// GET /relania
pub async fn index(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let tera = &data.tera;
    let conn = &data.conn;
    let ext = { req.extensions() };

    let mut context = ext.get::<Context>().unwrap_or(&Context::new()).to_owned();

    let Some(claims) = ext.get::<RdbClaims>() else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    };

    let characters = CharacterRepo::get_by_account_id(conn, claims.sub).await.unwrap();
    context.insert("characters", &characters);

    let Ok(html) = tera.render("relania/index.html", &context) else {
        return HttpResponse::InternalServerError().body("Template error");
    };

    HttpResponse::Ok().body(html)
}
