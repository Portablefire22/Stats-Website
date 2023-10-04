// Handle getting game information


use std::env;
use crate::api_structs::Summoner;
use crate::match_structs;
use crate::match_structs::MatchInformation;
use std::fs;

pub async fn get_match_details(local_summoner: &Summoner, game_id: String) -> MatchInformation{
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    let request_url: String = format!("https://{}.api.riotgames.com/lol/match/v5/matches/{}?api_key={}",local_summoner.routing_region, game_id, riot_api);
    let mut resp = reqwest::get(request_url).await.expect("Failed to get a response");
    let resp = resp.text().await.expect("Could not parse");
    let local_match: MatchInformation = serde_json::from_str(&resp).unwrap();
    let t = format!("{:#?}", local_match);
    fs::write("../test", t).expect("Unable to write to file");
    return local_match
}

pub async fn get_matches(local_summoner: &Summoner, match_ids: Vec<String>) -> Vec<MatchInformation> {
    let mut matches: Vec<MatchInformation> = Vec::new();
    for match_id in match_ids {
        matches.push(get_match_details(local_summoner, match_id).await);
    }
    let t = format!("{:#?}", matches);
    fs::write("test.test", t).expect("Unable to write vector to file");
    return matches;
}