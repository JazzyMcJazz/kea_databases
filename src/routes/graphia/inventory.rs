use actix_web::{web, HttpRequest, HttpResponse, http::Method};

use crate::{server::AppState, utils::{extensions::Extensions, claims::GdbClaims}, repo::gdbms::{character_repo::CharacterRepo, item_piece_repo::ItemPieceRepo}};


// POST /graphia/c/{character_id}/i
pub async fn loot_list(
    data: web::Data<AppState>,
    path: web::Path<(String,)>,
    req: HttpRequest,
) -> HttpResponse {
    let conn = &data.gdbms_surreal;
    let claims = Extensions::unwrap_claims::<GdbClaims, String>(&req);
    let character_id = &path.0;

    let Ok(is_owner) = CharacterRepo::exists_by_id_and_account_id(conn, &character_id, &claims.sub).await
    else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };

    if !is_owner {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    match ItemPieceRepo::fn_drop_loot(conn, character_id).await {
        Ok(_) => {}
        Err(e) => {
            dbg!(e);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    }

    HttpResponse::Created()
        .append_header(("HX-Refresh", "true"))
        .finish()
}

// PUT+PATCH+DELETE /graphia/c/{id}/i/{item_id}
pub async fn loot_details(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> HttpResponse {
    let conn = &data.gdbms_surreal;
    let claims = Extensions::unwrap_claims::<GdbClaims, String>(&req);
    let (character_id, item_id) = (path.0.to_owned(), path.1.to_owned());


    let Ok(is_owner) = CharacterRepo::exists_by_id_and_account_id(conn, &character_id, &claims.sub).await
    else {
        return HttpResponse::Unauthorized().body("Internal Server Error");
    };

    if !is_owner {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    match *req.method() {
        Method::PUT => {
            let _ = ItemPieceRepo::equip_by_id(conn, &character_id, &item_id).await;
        }
        Method::PATCH => {
            let _ = ItemPieceRepo::unequip_by_id(conn, &character_id, &item_id).await;
        }
        Method::DELETE => {
            let _ = ItemPieceRepo::delete_by_id(conn, &character_id, &item_id).await;
        }
        _ => {}
    }

    HttpResponse::Found()
        .append_header(("HX-Refresh", "true"))
        .finish()
}