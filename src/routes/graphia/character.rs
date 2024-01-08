use std::collections::HashMap;

use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::{
    repo::gdbms::{character_repo::CharacterRepo, class_repo::ClassRepo, enums::{Slot, Rarity}},
    server::AppState,
    utils::{claims::GdbClaims, extensions::Extensions, traits::{Terafy, Thingify}},
};

type EquippedGear<'a> = HashMap::<Slot, HashMap<String, String>>;

// GET /graphia/c
pub async fn create_character_page(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let tera = &data.tera;
    let conn = &data.gdbms_surreal;
    let mut context = Extensions::unwrap_context(&req);

    let mut classes = ClassRepo::all(conn).await.unwrap();
    tera.terafy(&mut classes);
    context.insert("classes", &classes);

    let Ok(html) = tera.render("graphia/create_character.html", &context) else {
        return HttpResponse::InternalServerError().body("Template error");
    };

    HttpResponse::Ok().body(html)
}

#[derive(Deserialize)]
pub struct CreateCharacterForm {
    pub character_name: String,
    pub class_id: String,
}

// POST /graphia/c
pub async fn create_character(
    data: web::Data<AppState>,
    req: HttpRequest,
    form: web::Form<CreateCharacterForm>,
) -> HttpResponse {
    let db = &data.gdbms_surreal;

    let claims = Extensions::unwrap_claims::<GdbClaims, String>(&req);

    let character_id =
        match CharacterRepo::create(db, &form.character_name, &form.class_id, &claims.sub).await {
            Ok(id) => id,
            Err(e) => return HttpResponse::InternalServerError().body(e),
        };

    HttpResponse::Found()
        .append_header(("HX-Redirect", format!("/graphia/c/{}", character_id)))
        .finish()
}

// GET+DELETE /graphia/c/{id}
pub async fn character_detail(
    data: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(String,)>,
) -> HttpResponse {
    let tera = &data.tera;
    
    let conn = &data.gdbms_surreal;
    let (claims, mut context) = Extensions::unwrap_claims_and_context::<GdbClaims, String>(&req);

    let id = &path.0;

    if req.method() == "DELETE" {
        let result = CharacterRepo::delete_by_id(conn, id).await;
        return HttpResponse::Found()
            .append_header(("HX-Redirect", "/graphia"))
            .finish();
    }

    let character = CharacterRepo::get_by_id(conn, id).await.expect("Error getting character");
    let Some(mut character) = character else {
        return HttpResponse::NotFound().body("Not found");
    };

    character.thingify();

    if character.account_id != claims.sub {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    // dbg!(&character);

    let (occupied_slots, equipped_gear) = {
        let mut slots: Vec<&Slot> = vec![];
        let mut map = new_gear_overview();
        // "Mainhand", "Offhand", "Head", "Chest", "Hands", "Legs", "Feet"
        for item in character.equipped_gear.iter() {
            slots.push(&item.slot);

            let mut item_data = HashMap::<String, String>::new();
            
            let color = match item.rarity {
                Rarity::Common => "grey",
                Rarity::Rare => "skyblue",
                Rarity::Epic => "gold",
                Rarity::Legendary => "purple",
            };

            item_data.insert("name".to_owned(), item.name.clone());
            item_data.insert("color".to_owned(), color.to_owned());
            map.insert(item.slot.clone(), item_data);
        }

        (slots, map)
    };

    for item in character.inventory.iter_mut() {
        // println!("{:#?}", item.slot);
        // println!("{:#?}", occupied_slots);
        // println!("{:#?}", occupied_slots.contains(&item.slot.as_str()));
        // println!("");
        item.can_equip = Some(!occupied_slots.contains(&&item.slot));
    }
    
    // dbg!(&equipped_gear);

    context.insert("character", &character);
    context.insert("equipped_gear", &equipped_gear);
    context.insert("is_owner", &(character.account_id == claims.sub));

    let html = match tera.render("graphia/character.html", &context) {
        Ok(html) => html,
        Err(e) => {
            dbg!(e);
            return HttpResponse::InternalServerError().body("Template error");
        }
    };

    HttpResponse::Ok().body(html)
}


// Helpers

fn new_gear_overview<'a>() -> EquippedGear<'a> {
    let mut map = EquippedGear::new();
    // "Mainhand", "Offhand", "Head", "Chest", "Hands", "Legs", "Feet"
    let slots = [
        Slot::Head,
        Slot::Chest,
        Slot::Hands,
        Slot::Legs,
        Slot::Feet,
        Slot::MainHand,
        Slot::OffHand,
    ];
    for slot in slots.iter() {
        let mut item_data = HashMap::<String, String>::new();

        item_data.insert("name".to_owned(), "none".to_owned());
        item_data.insert("color".to_owned(), "white".to_owned());
        map.insert(slot.clone(), item_data);
    }

    map
}