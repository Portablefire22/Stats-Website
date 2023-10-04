// Handle getting game information


use std::env;
use crate::api_structs::Summoner;
use crate::match_structs;
use crate::match_structs::MatchInformation;
use std::fs;

pub async fn get_match_details(local_summoner: &Summoner, game_id: &Vec<String>) -> MatchInformation{

    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    let request_url: String = format!("https://{}.api.riotgames.com/lol/match/v5/matches/{}?api_key={}",local_summoner.routing_region, game_id[0], riot_api);
    println!("{}", request_url);
    let mut resp = reqwest::get(request_url).await.expect("Failed to get a response");
    let resp = resp.text().await.expect("Could not parse");
    let local_match: MatchInformation = serde_json::from_str(&resp).unwrap();
    let t = format!("{:#?}", local_match);
    fs::write("../test", t).expect("Unable to write to file");
    return local_match
}


// Now that we have the match details, it's time to process them into a simplified version to send to the client I guess.
