#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use std::collections::HashMap;


#[get("/")]
fn index(
    cookies: Cookies
) -> Result<Template, Redirect> {
    // Redirect unauthorized users to th login page
    if !is_user_logged_in(&cookies) {
        Err(Redirect::found("/login"))
    } else {
        let mut context = HashMap::new();
        context.insert("username", cookies.get("username").unwrap().value());

        Ok(Template::render("index", context))
    }
}


#[get("/login")]
fn login_form(
    cookies: Cookies
) -> Result<Template, Redirect> {
    // Redirect already logged in user to the index page
    if is_user_logged_in(&cookies) {
        Err(Redirect::found("/"))
    } else {
        let context: HashMap<&str, &str> = HashMap::new();

        Ok(Template::render("login", context))
    }
}


#[derive(FromForm)]
struct LoginData {
    username: String,
    password: String,
}

#[post("/login", data = "<login_data>")]
fn login_verify(
    login_data: Form<LoginData>,
    mut cookies: Cookies
) -> Result<Template, Redirect> {
    // Redirect already logged in user to the index page
    if is_user_logged_in(&cookies) {
        Err(Redirect::to("/"))
    } else if !is_login_data_valid(&login_data) {
        let mut context = HashMap::new();
        context.insert("error", "Invalid credentials!");
        context.insert("username", &login_data.username);

        Ok(Template::render("login", context))
    } else {
        cookies.add(Cookie::new("username", login_data.into_inner().username));

        Err(Redirect::to("/"))
    }
}


#[get("/logout")]
fn logout(
    mut cookies: Cookies
) -> Redirect {
    cookies.remove(Cookie::named("username"));

    Redirect::to("/login")
}


fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, login_form, login_verify, logout])
        .launch();
}

fn is_login_data_valid(login_data: &Form<LoginData>) -> bool {
    login_data.username == "jlk" && login_data.password != ""
}

fn is_user_logged_in(cookies: &Cookies) -> bool {
    cookies.get("username").is_some()
}

