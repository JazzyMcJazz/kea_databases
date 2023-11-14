use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    repo::ddbms::character_repo::CharacterRepo,
    server::AppState,
    utils::{claims::DdbClaims, extensions::Extensions, traits::Terafy},
};

// GET /documenia
pub async fn index(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let tera = &data.tera;
    let db = &data.surreal;

    let (claims, mut context) = Extensions::unwrap_claims_and_context::<DdbClaims, String>(&req);

    let mut characters = CharacterRepo::get_by_account_id(db, claims.sub)
        .await
        .unwrap_or_else(|e| {
            dbg!(e);
            vec![]
        });

    tera.terafy(&mut characters);

    context.insert("characters", &characters);

    let Ok(html) = tera.render("documenia/index.html", &context) else {
        return HttpResponse::InternalServerError().body("Template error");
    };

    HttpResponse::Ok().body(html)
}
