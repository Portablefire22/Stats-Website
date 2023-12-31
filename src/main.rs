#![feature(future_join)]

use std::env;
use std::path::{Path, PathBuf};
use dotenvy::dotenv;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::Redirect;
use rocket::uri;

use syn;
use serde::{Deserialize, Serialize};

mod api_structs;
mod summoner_controller;
mod game_controller;
mod match_structs;
mod rune_structs;
mod item_structs;
mod item_controller;
mod analyse_summoner;
mod timeline_structs;


#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, tera::Tera, context};
use urlencoding::encode;
use crate::analyse_summoner::match_analysis;
use crate::game_controller::get_matches;
use crate::summoner_controller::get_match_history;
use crate::timeline_structs::Timeline;

// ------- Summoner Searching


// Get a Summoner from a given username and region
#[get("/<region>/<username>")]
async fn user_profile(region: &str, username: &str) -> Template {
    let local_summoner: api_structs::Summoner = summoner_controller::get_summoner_by_username(region, username).await;
    let matches = get_matches(&local_summoner, get_match_history(&local_summoner, 0, 2).await).await;
    Template::render("profile", context! {
        summoner: &local_summoner,
        profile_icon: &local_summoner.summoner_info.profile_icon_id,
        summoner_level: &local_summoner.summoner_info.summoner_level,
        match_history: matches,
        region: region,
    })
}

#[get("/<region>/<match_id>?<summoner_name..>")]
async fn match_showcase(region: &str, match_id: &str, summoner_name: &str) -> Template {
    dbg!(region, match_id, summoner_name);
    let local_summoner: api_structs::Summoner = summoner_controller::get_summoner_by_username(region, summoner_name).await;
    let mat = match_analysis(match_id, &local_summoner).await;
    Template::render("match", context!{
        focused_summoner: &local_summoner,
        match_id: match_id,
    })
}

#[derive(FromForm)]
struct SearchSummoner {
    username: String,
    region: String,
}
#[post("/", data="<summoner_info>")]
async fn search_input(summoner_info: rocket::form::Form<SearchSummoner>) -> Redirect {
    let safe_username = encode(&*summoner_info.username);
    let url: String = format!("/search/{}/{}", summoner_info.region, safe_username);
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
        .mount("/match/", routes![match_showcase])
        .mount("/public", FileServer::from("static/"))
        .attach(Template::fairing())

}


#[get("/")]
fn index() -> Template {
    Template::render("index", context! {test_value: "Testing"})
}

#[get("/datadragon/<file_path>")]
async fn datadragon(file_path: &str) -> Option<NamedFile> {
    NamedFile::open(format!("datadragon/{}", file_path)).await.ok()
}

#[get("/style.css")]
async fn stylesheet() -> Option<NamedFile> {
    println!("{:#?}", env::current_dir());
    NamedFile::open("templates/css/style.css").await.ok()
}