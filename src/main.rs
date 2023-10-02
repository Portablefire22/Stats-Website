use std::env;
use dotenvy::dotenv;
use syn;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str{
    "Hello, World!"
}

#[get("/<region>/<username>")]
async fn user_profile(region: &str, username: &str) -> String{

    format!("Region: {}\nUsername: {}", region, username);
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    println!("{}", riot_api);
    let request_url: String = format!("https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}?api_key={}",region, username, &riot_api);
    let resp = reqwest::get(request_url).await.expect("Failed to get a response").text().await.expect("Could not parse");
    let model: Summoner = serde_json::from_str(&resp).unwrap();
    format!("{:#?}", model)
}

#[launch]
fn rocket() -> _ {
    dotenv().expect(".env file not found");
    rocket::build()
        .mount("/", routes![index])
        .mount("/search/", routes![userProfile])

}



#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    id: String,
    account_id: String,
    puuid: String,
    name: String,
    profile_icon_id: i64,
    revision_date: i64,
    summoner_level: i64,
}
