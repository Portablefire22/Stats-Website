use std::env;
use std::path::{Path, PathBuf};
use dotenvy::dotenv;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::Redirect;

use syn;
use serde::{Deserialize, Serialize};

mod api_structs;
mod summoner_controller;
#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, tera::Tera, context};


// ------- Summoner Searching


// Get a Summoner from a given username and region
#[get("/<region>/<username>")]
async fn user_profile(region: &str, username: &str) -> Template {
    let local_summoner: api_structs::Summoner = summoner_controller::get_summoner(region, username).await;
    Template::render("profile", context! {summoner: &local_summoner})
}

#[derive(FromForm)]
struct SearchSummoner {
    username: String,
    region: String,
}
#[post("/", data="<summoner_info>")]
async fn search_input(summoner_info: rocket::form::Form<SearchSummoner>) -> Redirect {
    let url: String = format!("/search/{}/{}", summoner_info.region, summoner_info.username);
    Redirect::to(url)
}

// -------- End of Summoner Searching


#[launch]
fn rocket() -> _ {
    dotenv().expect(".env file not found");
    rocket::build()
        .mount("/", routes![index])
        .mount("/search/", routes![user_profile, search_input])
        .mount("/css/", routes![stylesheet])
        .mount("/public", FileServer::from("static/"))
        .attach(Template::fairing())

}


#[get("/")]
fn index() -> Template {
    Template::render("index", context! {test_value: "Testing"})
}


#[get("/style.css")]
async fn stylesheet() -> Option<NamedFile> {
    println!("{:#?}", env::current_dir());
    NamedFile::open("templates/css/style.css").await.ok()
}