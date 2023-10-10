use actix_web::{web, HttpRequest, HttpResponse, HttpMessage, http::Method};

use crate::{server::AppState, repo::rdbms::{item_piece_repo::ItemPieceRepo, character_repo::CharacterRepo}, utils::auth::RdbClaims};

// POST /relania/c/{id}/i
pub async fn loot_list(data: web::Data<AppState>, path: web::Path<(i32,)>, req: HttpRequest) -> HttpResponse {
    let conn = &data.conn;
    let ext = { req.extensions() };

    let Some(claims) = ext.get::<RdbClaims>() else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    };

    let Ok(is_owner) = CharacterRepo::exists_by_id_and_account_id(&conn, path.0, claims.sub).await else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    };

    if !is_owner {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    let _ = ItemPieceRepo::sp_drop_loot(conn, path.0).await;

    HttpResponse::Created()
        .append_header(("HX-Refresh", "true"))
        .finish()
}

// PUT+PATCH+DELETE /relania/c/{id}/i/{loot_id}
pub async fn loot_details(data: web::Data<AppState>, path: web::Path<(i32, i32)>, req: HttpRequest) -> HttpResponse {
    let conn = &data.conn;
    let ext = { req.extensions() };

    let Some(claims) = ext.get::<RdbClaims>() else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    };

    let Ok(is_owner) = CharacterRepo::exists_by_id_and_account_id(&conn, path.0, claims.sub).await else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    };

    if !is_owner {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }


    match req.method() {
        &Method::PUT => {
            let _ = ItemPieceRepo::equip_by_id(conn, path.0, path.1).await;
        },
        &Method::PATCH => {
            // let _ = ItemPieceRepo::unequip_by_id(conn, path.0, path.1).await;
        },
        &Method::DELETE => {
            let _ = ItemPieceRepo::delete_by_id(conn, path.1).await;
        },
        _ => {}
    }

    HttpResponse::Found()
        .append_header(("HX-Refresh", "true"))
        .finish()
}