use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    repo::rdbms::character_repo::CharacterRepo,
    server::AppState,
    utils::{claims::RdbClaims, extensions::Extensions},
};

// GET /relania
pub async fn index(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let tera = &data.tera;
    let conn = &data.conn;
    let (claims, mut context) = Extensions::unwrap_claims_and_context::<RdbClaims, i32>(&req);

    let characters = CharacterRepo::get_by_account_id(conn, claims.sub)
        .await
        .unwrap();
    context.insert("characters", &characters);

    let Ok(html) = tera.render("relania/index.html", &context) else {
        return HttpResponse::InternalServerError().body("Template error");
    };

    HttpResponse::Ok().body(html)
}
