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


// Right, so I can either go full backend and do front end later,
// or do front end right now. Actually not quite sure which one I should
// be doing or want to do tbh.

// Lets focus on the back end
// No point in making an Error 500 look pretty
#[get("/<region>/<username>")]
async fn user_profile(region: &str, username: &str) -> &'static str {
    format!("Region: {}\nUsername: {}", region, username);
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    println!("{}", riot_api);
    let request_url: String = format!("https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}?api_key={}",region, username, &riot_api);
    let resp = reqwest::get(request_url).await.expect("Failed to get a response").text().await.expect("Could not parse");
    let mut local_summoner_info: SummonerInfo = serde_json::from_str(&resp).unwrap();
    let local_summoner: Summoner = Summoner {
        summoner_info: local_summoner_info,
        region: region.parse().unwrap(),
    };
    testing_function(&local_summoner).await;
    return "String" // Returns the profile as a Summoner struct
}

async fn testing_function(local_summoner: &Summoner) {
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    let request_url: String = format!("https://{}.api.riotgames.com/lol/league/v4/entries/by-summoner/{}?api_key={}", local_summoner.region, local_summoner.summoner_info.id, riot_api);
    println!("{:#?}", request_url);
    let mut resp = reqwest::get(request_url).await.expect("Failed to get a response").text().await.expect("Could not parse");
    println!("{:#?}", resp[0]); // Im eepy sleep, i leave this to future me
    resp = serde_json::from_str(&resp[0]).unwrap();
    println!("{}", resp);
}

#[launch]
fn rocket() -> _ {
    dotenv().expect(".env file not found");
    rocket::build()
        .mount("/", routes![index])
        .mount("/search/", routes![user_profile])

}



#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SummonerInfo {
    id: String,
    account_id: String,
    puuid: String,
    name: String,
    profile_icon_id: i64,
    revision_date: i64,
    summoner_level: i64,
}

pub struct Summoner {
    summoner_info: SummonerInfo,
    region: String,
}
