use actix_web::{http::Method, web, HttpRequest, HttpResponse};

use crate::{
    repo::rdbms::{character_repo::CharacterRepo, item_piece_repo::ItemPieceRepo},
    server::AppState,
    utils::{claims::RdbClaims, extensions::Extensions},
};

// POST /relania/c/{id}/i
pub async fn loot_list(
    data: web::Data<AppState>,
    path: web::Path<(i32,)>,
    req: HttpRequest,
) -> HttpResponse {
    let conn = &data.conn;
    let claims = Extensions::unwrap_claims::<i32, RdbClaims>(&req);

    let Ok(is_owner) = CharacterRepo::exists_by_id_and_account_id(conn, path.0, claims.sub).await
    else {
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
pub async fn loot_details(
    data: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
    req: HttpRequest,
) -> HttpResponse {
    let conn = &data.conn;
    let claims = Extensions::unwrap_claims::<i32, RdbClaims>(&req);

    let Ok(is_owner) = CharacterRepo::exists_by_id_and_account_id(conn, path.0, claims.sub).await
    else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    };

    if !is_owner {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    match *req.method() {
        Method::PUT => {
            let _ = ItemPieceRepo::equip_by_id(conn, path.0, path.1).await;
        }
        Method::PATCH => {
            // let _ = ItemPieceRepo::unequip_by_id(conn, path.0, path.1).await;
        }
        Method::DELETE => {
            let _ = ItemPieceRepo::delete_by_id(conn, path.1).await;
        }
        _ => {}
    }

    HttpResponse::Found()
        .append_header(("HX-Refresh", "true"))
        .finish()
}
