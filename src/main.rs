use actix_web::{get, post, web, http, App, HttpResponse, HttpServer, Responder};
use tera::Tera;
use serde::Deserialize;


#[get("/")]
async fn index(
    tmpl: web::Data<tera::Tera>
) -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        tmpl.render("index.html", &tera::Context::new()).unwrap()
    )
}


#[get("/login")]
async fn login_form(
    tmpl: web::Data<tera::Tera>
) -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        tmpl.render("login.html", &tera::Context::new()).unwrap()
    )
}


#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[post("/login")]
async fn login_verify(
    tmpl: web::Data<tera::Tera>,
    login_data: web::Form<LoginData>
) -> impl Responder {
    if is_login_data_valid(&login_data) {
        HttpResponse::Found().header(http::header::LOCATION, "/").finish()
    } else {
        let mut context = tera::Context::new();
        context.insert("error", "Invalid credentials!");
        context.insert("username", &login_data.username);
        HttpResponse::Unauthorized().content_type("text/html").body(
            tmpl.render("login.html", &context).unwrap()
        )
    }
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
            .service(index)
            .service(login_form)
            .service(login_verify)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn is_login_data_valid(login_data: &web::Form<LoginData>) -> bool {
    login_data.username == "jlk"
}
