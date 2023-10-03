use std::env;
use std::path::{Path, PathBuf};
use dotenvy::dotenv;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::Redirect;

use syn;
use serde::{Deserialize, Serialize};

mod api_structs;
#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, tera::Tera, context};


// ------- Summoner Searching


// Get a Summoner from a given username and region
#[get("/<region>/<username>")]
async fn user_profile(region: &str, username: &str) -> String {
    format!("Region: {}\nUsername: {}", region, username);
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    println!("{}", riot_api);
    let request_url: String = format!("https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}?api_key={}",region, username, &riot_api);
    let resp = reqwest::get(request_url).await.expect("Failed to get a response").text().await.expect("Could not parse");
    let mut local_summoner_info: api_structs::SummonerInfo = serde_json::from_str(&resp).unwrap();
    let local_summoner: api_structs::Summoner = api_structs::Summoner {
        summoner_info: local_summoner_info,
        region: region.parse().unwrap(),
    };
    format!("{:#?}", &local_summoner) // Returns the profile as a Summoner struct
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

// Return the ranked information of a given summoner
async fn get_ranked_information(local_summoner: &api_structs::Summoner) -> api_structs::SummonerRanked{
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    let request_url: String = format!("https://{}.api.riotgames.com/lol/league/v4/entries/by-summoner/{}?api_key={}", local_summoner.region, local_summoner.summoner_info.id, riot_api);
    println!("{:#?}", request_url);
    let resp = reqwest::get(request_url).await.expect("Failed to get a response").text().await.expect("Could not parse");
    println!("{}", resp);
    let local_summoner_ranked: api_structs::SummonerRanked = serde_json::from_str(&resp).unwrap();
    println!("{:#?}", local_summoner_ranked);
    local_summoner_ranked
}

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