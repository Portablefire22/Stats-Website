use std::env;
use dotenvy::dotenv;
use syn;
use serde::{Deserialize, Serialize};


mod api_structs;
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
    format!("{:#?}", get_ranked_information(&local_summoner).await) // Returns the profile as a Summoner struct
}

async fn get_ranked_information(local_summoner: &api_structs::Summoner) -> api_structs::SummonerRanked{
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    let request_url: String = format!("https://{}.api.riotgames.com/lol/league/v4/entries/by-summoner/{}?api_key={}", local_summoner.region, local_summoner.summoner_info.id, riot_api);
    println!("{:#?}", request_url);
    let mut resp = reqwest::get(request_url).await.expect("Failed to get a response").text().await.expect("Could not parse");
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
        .mount("/search/", routes![user_profile])

}



