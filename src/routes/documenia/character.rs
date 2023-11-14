use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::{
    repo::ddbms::{character_repo::CharacterRepo, class_repo::ClassRepo},
    server::AppState,
    utils::{claims::DdbClaims, extensions::Extensions, traits::Terafy},
};

// GET /documenia/c
pub async fn create_character_page(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let tera = &data.tera;
    let conn = &data.surreal;
    let mut context = Extensions::unwrap_context(&req);

    let mut classes = ClassRepo::all(conn).await.unwrap();
    tera.terafy(&mut classes);
    context.insert("classes", &classes);

    let Ok(html) = tera.render("documenia/create_character.html", &context) else {
        return HttpResponse::InternalServerError().body("Template error");
    };

    HttpResponse::Ok().body(html)
}

#[derive(Deserialize)]
pub struct CreateCharacterForm {
    pub character_name: String,
    pub class_id: String,
}

// POST /documenia/c
pub async fn create_character(
    data: web::Data<AppState>,
    req: HttpRequest,
    form: web::Form<CreateCharacterForm>,
) -> HttpResponse {
    let db = &data.surreal;

    let claims = Extensions::unwrap_claims::<DdbClaims, String>(&req);

    let character_id =
        match CharacterRepo::create(db, &form.character_name, &form.class_id, &claims.sub).await {
            Ok(id) => id,
            Err(e) => return HttpResponse::InternalServerError().body(e),
        };

    HttpResponse::Found()
        .append_header(("HX-Redirect", format!("/documenia/c/{}", character_id)))
        .finish()
}

// GET+DELETE /documenia/c/{id}
pub async fn _character_detail(
    data: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(i32,)>,
) -> HttpResponse {
    let tera = &data.tera;
    
    let _conn = &data.conn;
    let (_claims, context) = Extensions::unwrap_claims_and_context::<DdbClaims, String>(&req);

    let _id = path.0;

    if req.method() == "DELETE" {
        // let _ = CharacterRepo::delete_by_id(conn, id).await;
        // return HttpResponse::Found()
        //     .append_header(("HX-Redirect", "/documenia"))
        //     .finish();
    }

    // let character = CharacterRepo::get_view_by_id(conn, id).await.unwrap();
    // let Some(character) = character else {
    //     return HttpResponse::NotFound().body("Not found");
    // };

    // if character.account_id != claims.sub {
    //     return HttpResponse::Unauthorized().body("Unauthorized");
    // }

    // let inventory = todo!();//InventoryRepo::find_item_pieces_by_inventory_id(conn, id).await;
    // let Ok(mut inventory) = inventory else {
    //     return HttpResponse::InternalServerError().body("Internal server error");
    // };

    // let occupied_slots = {
    //     // "Mainhand", "Offhand", "Head", "Chest", "Hands", "Legs", "Feet"
    //     let mut slots: Vec<&str> = vec![];
    //     if character.head_id.is_some() {
    //         slots.push("Head");
    //     }
    //     if character.chest_id.is_some() {
    //         slots.push("Chest");
    //     }
    //     if character.hands_id.is_some() {
    //         slots.push("Hands");
    //     }
    //     if character.legs_id.is_some() {
    //         slots.push("Legs");
    //     }
    //     if character.feet_id.is_some() {
    //         slots.push("Feet");
    //     }
    //     if character.mainhand_id.is_some() {
    //         slots.push("Mainhand");
    //     }
    //     if character.offhand_id.is_some() {
    //         slots.push("Offhand");
    //     }

    //     slots
    // };

    // for item in inventory.iter_mut() {
    //     // println!("{:#?}", item.slot);
    //     // println!("{:#?}", occupied_slots);
    //     // println!("{:#?}", occupied_slots.contains(&item.slot.as_str()));
    //     // println!("");
    //     item.can_equip = Some(!occupied_slots.contains(&item.slot.as_str()));
    // }

    // context.insert("character", &character);
    // context.insert("inventory", &inventory);
    // context.insert("is_owner", &character["account_id"] == claims.sub);

    let Ok(html) = tera.render("relania/character.html", &context) else {
        return HttpResponse::InternalServerError().body("Template error");
    };

    HttpResponse::Ok().body(html)
}
