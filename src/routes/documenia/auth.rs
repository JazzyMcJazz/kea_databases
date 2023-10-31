use actix_web::{
    cookie::{self, Cookie},
    web, HttpRequest, HttpResponse,
};
use jsonwebtoken::EncodingKey;
use serde::Deserialize;

use crate::{
    repo::ddbms::account_repo::SurrealAccountRepo,
    server::AppState,
    utils::{
        auth::{Claims, DdbClaims},
        extensions::Extensions,
    },
};

const COOKIE_NAME: &str = "ddb_id";
const LOGIN_ERROR: &str = "Invalid username or password";
const DEFAULT_REDIRECT: &str = "/documenia";

// GET /documenia/login
pub async fn login_page(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let tera = &data.tera;

    let context = Extensions::unwrap_context(&req);

    let Ok(html) = tera.render("documenia/auth/login.html", &context) else {
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
pub async fn login(
    data: web::Data<AppState>,
    form: web::Form<LoginForm>,
    q: web::Query<PathQuery>,
) -> HttpResponse {
    let db = &data.surreal;

    let Ok(acc) =
        SurrealAccountRepo::find_by_credentials(db, form.username.clone(), form.password.clone())
            .await
    else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };

    let Some(acc) = acc else {
        return HttpResponse::Unauthorized().body(LOGIN_ERROR);
    };

    let Ok(token) = auth_token(acc.id.id.to_string(), acc.username) else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };

    let cookie = Cookie::new(COOKIE_NAME, token);

    let location = match &q.next {
        Some(next) => next,
        None => DEFAULT_REDIRECT,
    };

    HttpResponse::Found()
        .append_header(("HX-Redirect", location))
        .cookie(cookie)
        .finish()
}

// GET /documenia/register
pub async fn register_page(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let tera = &data.tera;

    let context = Extensions::unwrap_context(&req);

    let Ok(html) = tera.render("documenia/auth/register.html", &context) else {
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

    let db = &data.surreal;
    let account =
        match SurrealAccountRepo::create_account(db, form.username.clone(), form.password.clone())
            .await
        {
            Ok(account) => account,
            Err(_) => {
                return HttpResponse::Conflict().body("Username already exists");
            }
        };

    let Ok(token) = auth_token(account.id.id.to_string(), form.username.clone()) else {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    };

    let cookie = Cookie::new(COOKIE_NAME, token);

    HttpResponse::Found()
        .append_header(("HX-Redirect", DEFAULT_REDIRECT))
        .cookie(cookie)
        .finish()
}

// GET /relania/logout
pub async fn logout(q: web::Query<PathQuery>) -> HttpResponse {
    let expires = cookie::time::OffsetDateTime::from_unix_timestamp(0).unwrap();

    let cookie = Cookie::build(COOKIE_NAME, "").expires(expires).finish();

    let location = match &q.next {
        Some(next) => next,
        None => DEFAULT_REDIRECT,
    };

    HttpResponse::Found()
        .append_header(("Location", location))
        .cookie(cookie)
        .finish()
}

fn auth_token(id: String, username: String) -> Result<String, ()> {
    let Ok(secret) = std::env::var("JWT_SECRET") else {
        eprintln!("Auth Error: JWT_SECRET is not set in .env file");
        return Err(());
    };

    let key = EncodingKey::from_secret(secret.as_ref());

    let claims = DdbClaims::new(id, username);

    let Ok(token) = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &key) else {
        eprintln!("Auth Error: Failed to encode JWT");
        return Err(());
    };

    Ok(token)
}
