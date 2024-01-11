use actix_web::{web, HttpRequest, HttpResponse};

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

// PUT+PATCH+DELETE /relania/c/{id}/i/{loot_id}
// pub async fn loot_details(
//     data: web::Data<AppState>,
//     path: web::Path<(i32, i32)>,
//     req: HttpRequest,
// ) -> HttpResponse {
//     let conn = &data.conn;
//     let claims = Extensions::unwrap_claims::<RdbClaims, i32>(&req);

//     let Ok(is_owner) = CharacterRepo::exists_by_id_and_account_id(conn, path.0, claims.sub).await
//     else {
//         return HttpResponse::Unauthorized().body("Unauthorized");
//     };

//     if !is_owner {
//         return HttpResponse::Unauthorized().body("Unauthorized");
//     }

//     match *req.method() {
//         Method::PUT => {
//             let _ = ItemPieceRepo::equip_by_id(conn, path.0, path.1).await;
//         }
//         Method::PATCH => {
//             // let _ = ItemPieceRepo::unequip_by_id(conn, path.0, path.1).await;
//         }
//         Method::DELETE => {
//             let _ = ItemPieceRepo::delete_by_id(conn, path.1).await;
//         }
//         _ => {}
//     }

//     HttpResponse::Found()
//         .append_header(("HX-Refresh", "true"))
//         .finish()
// }