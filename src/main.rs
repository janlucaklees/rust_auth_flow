use actix_web::{get, post, web, http, App, HttpResponse, HttpServer, Responder};
use actix_session::{CookieSession, Session};
use tera::Tera;
use serde::Deserialize;


#[get("/")]
async fn index(
    tmpl: web::Data<tera::Tera>,
    session: Session
) -> impl Responder {
    // Redirect unauthorized users to th login page
    if !is_user_logged_in(&session) {
        HttpResponse::Found().header(http::header::LOCATION, "/login").finish()
    } else {
        let mut context = tera::Context::new();
        context.insert("username", &session.get::<String>("username").unwrap().unwrap());
        HttpResponse::Ok().content_type("text/html").body(
            tmpl.render("index.html", &context).unwrap()
        )
    }
}


#[get("/login")]
async fn login_form(
    tmpl: web::Data<tera::Tera>,
    session: Session
) -> impl Responder {
    // Redirect already logged in user to the index page
    if is_user_logged_in(&session) {
        HttpResponse::Found().header(http::header::LOCATION, "/").finish()
    } else {
        HttpResponse::Ok().content_type("text/html").body(
            tmpl.render("login.html", &tera::Context::new()).unwrap()
        )
    }
}


#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[post("/login")]
async fn login_verify(
    tmpl: web::Data<tera::Tera>,
    login_data: web::Form<LoginData>,
    session: Session
) -> impl Responder {
    // Redirect already logged in user to the index page
    if is_user_logged_in(&session) {
        HttpResponse::Found().header(http::header::LOCATION, "/").finish()
    } else if !is_login_data_valid(&login_data) {
        let mut context = tera::Context::new();
        context.insert("error", "Invalid credentials!");
        context.insert("username", &login_data.username);
        HttpResponse::Unauthorized().content_type("text/html").body(
            tmpl.render("login.html", &context).unwrap()
        )
    } else {
        session.set("username", &login_data.username);
        HttpResponse::Found().header(http::header::LOCATION, "/").finish()
    }
}


#[get("/logout")]
async fn logout(
    tmpl: web::Data<tera::Tera>,
    session: Session
) -> impl Responder {
    session.purge();
    HttpResponse::Found().header(http::header::LOCATION, "/login").finish()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {

        let tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        App::new()
            .data(tera)
            .wrap(
                CookieSession::signed(&[0; 32])
                    .secure(false)
            )
            .service(index)
            .service(login_form)
            .service(login_verify)
            .service(logout)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn is_login_data_valid(login_data: &web::Form<LoginData>) -> bool {
    login_data.username == "jlk" && login_data.password != ""
}

fn is_user_logged_in(session: &Session) -> bool {
    session.get::<String>("username").unwrap().is_some()
}

