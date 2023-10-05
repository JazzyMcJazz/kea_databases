use actix_web::{HttpMessage, HttpResponse, HttpRequest, web, cookie, cookie::Cookie};
use sea_orm::{Set, EntityTrait, QueryFilter};
use jsonwebtoken::{self, EncodingKey};
use migration::sea_orm::*;
use serde::Deserialize;
use tera::Context;
use chrono;

use crate::entity::account;
use crate::server::AppState;
use crate::utils::auth::RdbClaims;

const COOKIE_NAME: &str = "rdb_id";
const LOGIN_ERROR: &str = "Invalid username or password";
const DEFAULT_REDIRECT: &str = "/relania";

// GET /relania/login
pub async fn login_page(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let tera = &data.tera;
    let ext = { req.extensions() };

    let context = ext.get::<Context>().unwrap_or(&Context::new()).to_owned();

    let Ok(html) = tera.render("relania/auth/login.html", &context) else {
        return HttpResponse::InternalServerError().body("Template error");
    };

    HttpResponse::Ok().body(html)
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct PathQuery {
    next: Option<String>,
}

// POST /relania/login
pub async fn login(data: web::Data<AppState>, form: web::Form<LoginForm>, q: web::Query<PathQuery>) -> HttpResponse {
    let conn = &data.conn;

    let Ok(acc) = account::Entity::find()
        .filter(account::Column::Username.eq(&form.username))
        .one(conn).await else {
        return HttpResponse::Unauthorized().body(LOGIN_ERROR);
    };

    let Some(acc) = acc else {
        return HttpResponse::Unauthorized().body(LOGIN_ERROR);
    };

    let Ok(password_match) = bcrypt::verify(&form.password, &acc.password) else {
        return HttpResponse::Unauthorized().body(LOGIN_ERROR);
    };

    if !password_match {
        return HttpResponse::Unauthorized().body(LOGIN_ERROR);
    }

    let Ok(token) = auth_token(acc.id, acc.username) else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };

    let cookie = Cookie::new(COOKIE_NAME, token);

    let location = match &q.next {
        Some(next) => next,
        None => DEFAULT_REDIRECT,
    };

    HttpResponse::Found()
        .append_header(("Location", location))
        .cookie(cookie)
        .finish()
}

// GET /relania/register
pub async fn register_page(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let tera = &data.tera;
    let ext = { req.extensions() };

    let context = ext.get::<Context>().unwrap_or(&Context::new()).to_owned();

    let Ok(html) = tera.render("relania/auth/register.html", &context) else {
        return HttpResponse::InternalServerError().body("Template error");
    };

    HttpResponse::Ok().body(html)
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub password: String,
    pub re_password: String,
}

// POST /relania/register
pub async fn register(data: web::Data<AppState>, form: web::Form<RegisterForm>) -> HttpResponse {
    if form.username.len() < 3 {
        return HttpResponse::BadRequest().body("Username must be at least 3 characters long");
    }

    if form.password != form.re_password {
        return HttpResponse::BadRequest().body("Passwords do not match");
    }

    if form.password.len() < 6 {
        return HttpResponse::BadRequest().body("Password must be at least 6 characters long");
    }

    let current_time = chrono::Utc::now();

    let password_hash = bcrypt::hash(&form.password, bcrypt::DEFAULT_COST).unwrap();

    let acc = account::ActiveModel {
        username: Set(form.username.clone()),
        password: Set(password_hash),
        last_login: Set(current_time),
        ..Default::default()
    };

    let conn = &data.conn;
    let Ok(result) = account::Entity::insert(acc).exec(conn).await else {
        return HttpResponse::InternalServerError().body("Username already exists");
    };

    let Ok(token) = auth_token(result.last_insert_id, form.username.clone()) else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };

    let cookie = Cookie::new(COOKIE_NAME, token);

    HttpResponse::Found()
        .append_header(("Location", DEFAULT_REDIRECT))
        .cookie(cookie)
        .finish()
}

// GET /relania/logout
pub async fn logout(q: web::Query<PathQuery>) -> HttpResponse {
    let expires = cookie::time::OffsetDateTime::from_unix_timestamp(0).unwrap();

    let cookie = Cookie::build(COOKIE_NAME, "")
        .expires(expires)
        .finish();

    let location = match &q.next {
        Some(next) => next,
        None => DEFAULT_REDIRECT,
    };

    HttpResponse::Found()
        .append_header(("Location", location))
        .cookie(cookie)
        .finish()
}

fn auth_token(id: i32, username: String) -> Result<String, ()> {
    let Ok(secret) = std::env::var("JWT_SECRET") else {
        eprintln!("Auth Error: JWT_SECRET is not set in .env file");
        return Err(());
    };
    
    let key = EncodingKey::from_secret(secret.as_ref());
    
    let claims = RdbClaims::new(id, username);

    let Ok(token) = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &key) else {
        eprintln!("Auth Error: Failed to encode JWT");
        return Err(());
    };

    Ok(token)
}