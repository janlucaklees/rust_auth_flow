use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tera::Tera;

#[get("/")]
async fn index(tmpl: web::Data<tera::Tera>) -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        tmpl.render("index.html", &tera::Context::new()).unwrap()
    )
}

#[post("/")]
async fn index_post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
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
            .service(index_post)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
