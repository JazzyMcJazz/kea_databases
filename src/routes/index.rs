use actix_web::{web, HttpRequest, HttpResponse};
use rand::Rng;

use crate::{server::AppState, utils::extensions::Extensions};

pub async fn index(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let tera = &data.tera;

    let context = Extensions::unwrap_context(&req);

    let Ok(html) = tera.render("index.html", &context) else {
        return HttpResponse::InternalServerError().body("Template error");
    };

    HttpResponse::Ok().body(html)
}

pub async fn not_found(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let tera = &data.tera;

    let possible_messages = [
        "I'm sorry Dave, I'm afraid I can't do that.",
        "I'm sorry, but you seem to be lost.",
        "These aren't the droids you're looking for.",
        "What you seek is not here.",
        "I have a bad feeling about this.",
        "We seem to have hit a snag.",
        "Whoopsie daisy!",
        "Looks like the page took a day off... and forgot to tell us.",
        "We'd say 'you broke the internet', but someone else claimed that one.",
        "We're great at many things. Clearly, finding this page isn't one of them.",
        "This page is playing hide and seek. And it's winning.",
        "The page you're looking for is on a mission to Mars. Want to wait or try a different page?",
        "Did you try turning the internet off and on again?",
        "Error 404: Page not found. But on the bright side, you found this funny message!",
        "Somewhere, a hamster stopped running and powered down our server. We're on it!",
        "This isn’t the page you’re looking for. Maybe it's with those missing socks?",
        "Congratulations! You've ventured into the void of the internet. Please turn back now."
    ];

    let i = rand::thread_rng().gen_range(0..possible_messages.len());
    let message = possible_messages[i];

    let mut context = Extensions::unwrap_context(&req);
    context.insert("message", &message);

    let Ok(html) = tera.render("404.html", &context) else {
        return HttpResponse::InternalServerError().body("Template error");
    };

    HttpResponse::Ok().body(html)
}
