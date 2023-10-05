use actix_web::{web, HttpRequest, HttpResponse, HttpMessage};
use serde::Deserialize;
use tera::Context;

use crate::{server::AppState, utils::auth::RdbClaims, repo::rdbms::{character_repo::CharacterRepo, class_repo::ClassRepo}};

// GET /relania/c
pub async fn create_character_view(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let tera = &data.tera;
    let conn = &data.conn;
    let ext = { req.extensions() };

    let classes = ClassRepo::all(conn).await.unwrap();
    let mut context = ext.get::<Context>().unwrap_or(&Context::new()).to_owned();
    context.insert("classes", &classes);

    let Ok(html) = tera.render("relania/create_character.html", &context) else {
        return HttpResponse::InternalServerError().body("Template error");
    };

    HttpResponse::Ok().body(html)
}

#[derive(Deserialize)]
pub struct CreateCharacterForm {
    pub character_name: String,
    pub class_id: i32,
}

// POST /relania/c
pub async fn create_character(data: web::Data<AppState>, req: HttpRequest, form: web::Form<CreateCharacterForm>) -> HttpResponse {
    let conn = &data.conn;
    let ext = { req.extensions() };

    let Some(claims) = ext.get::<RdbClaims>() else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    };

    let Ok(character_id) = CharacterRepo::create(conn, &form.character_name, form.class_id, claims.sub).await else {
        return HttpResponse::Conflict().body("Character name is unavailable");
    };

    HttpResponse::Found()
        .append_header(("HX-Redirect", format!("/relania/c/{}", character_id)))
        .finish()
}

// GET /relania/c/{id}
pub async fn character_list(data: web::Data<AppState>, req: HttpRequest, path: web::Path<(i32,)>) -> HttpResponse {
    let tera = &data.tera;
    let conn = &data.conn;
    let ext = { req.extensions() };

    let mut context = ext.get::<Context>().unwrap_or(&Context::new()).to_owned();

    let Some(claims) = ext.get::<RdbClaims>() else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    };

    let id = path.0;
    let character = CharacterRepo::get_view_by_id(conn, id).await.unwrap();
    let Some(character) = character else {
        return HttpResponse::NotFound().body("Not found");
    };

    if character.account_id != claims.sub {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    context.insert("character", &character);
    // context.insert("is_owner", &character["account_id"] == claims.sub);

    let Ok(html) = tera.render("relania/character.html", &context) else {
        return HttpResponse::InternalServerError().body("Template error");
    };

    HttpResponse::Ok().body(html)
}