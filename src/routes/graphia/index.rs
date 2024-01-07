use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    repo::gdbms::character_repo::CharacterRepo,
    server::AppState,
    utils::{claims::GdbClaims, extensions::Extensions, traits::Terafy},
};

// GET /graphia
pub async fn index(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let tera = &data.tera;
    let db = &data.gdbms_surreal;

    let (claims, mut context) = Extensions::unwrap_claims_and_context::<GdbClaims, String>(&req);

    let mut characters = CharacterRepo::get_by_account_id(db, claims.sub)
        .await
        .unwrap_or_else(|e| {
            dbg!(e);
            vec![]
        });

    tera.terafy(&mut characters);

    context.insert("characters", &characters);

    let Ok(html) = tera.render("graphia/index.html", &context) else {
        return HttpResponse::InternalServerError().body("Template error");
    };

    HttpResponse::Ok().body(html)
}
