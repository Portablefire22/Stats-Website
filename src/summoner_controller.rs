use std::env;
use reqwest::{Response, StatusCode};
use rocket::http::uri::{Absolute, Origin, Uri};
use serde_json::from_str;
use crate::api_structs;
use crate::api_structs::Summoner;
use crate::game_controller::get_match_details;

// Get the summoner information
pub(crate) async fn get_summoner_by_username(region: &str, username: &str) -> Summoner {
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");

    let mut request_url: String = format!("https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}?api_key={}",region, username, &riot_api);
    let request_url: Uri = Uri::parse::<Absolute>(&*request_url).expect("Failed to parse");
    let mut resp = reqwest::get(request_url.to_string()).await.expect("Failed to get a response");
    let status :StatusCode = resp.status();
    println!("{:#?}", status);
    if status == 200 {
        return create_summoner(region, resp, status).await;
    }
    let mut default_summoner: Summoner = api_structs::Summoner::default();
    default_summoner.debug_status = u16::from(status);
    return default_summoner;
}

pub(crate) async fn get_summoner_by_puuid(region: &str, puuid: &str) -> Summoner {
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    let mut request_url: String = format!("https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/{}?api_key={}", region, puuid, &riot_api);
    let request_url: Uri = Uri::parse::<Absolute>(&*request_url).expect("Failed to parse");
    println!("URL = {:#?}", request_url);
    let mut resp = reqwest::get(&request_url.to_string()).await.expect("Failed to get a response");
    let status :StatusCode = resp.status();
    println!("{:#?}", status);
    if status == 200 {
        return create_summoner(region, resp, status).await;
    }
    let mut default_summoner: Summoner = api_structs::Summoner::default();
    default_summoner.debug_status = u16::from(status);

    return default_summoner;
}

async fn create_summoner(region: &str, resp: Response, status: StatusCode) -> Summoner {
    let resp: String = resp.text().await.expect("Could not parse");
    let mut local_summoner_info: api_structs::SummonerInfo = from_str(&resp).unwrap();
    let mut local_summoner = Summoner {
        summoner_info: local_summoner_info,
        region: region.parse().unwrap(),
        routing_region: match &*(region).to_uppercase() { // Some api calls require a routing region and not just the summoner's server
            ("NA1" | "BR1" | "LA1" | "LA2") => String::from("AMERICA"),
            ("EUW1" | "EUN1" | "TR1" | "RU") => String::from("EUROPE"),
            ("KR" | "JP1") => String::from("ASIA"),
            ("PH21" | "SG2" | "TH2" | "TW2" | "VN2") => String::from("SEA"),
            _ => unreachable!()
        },
        ranked_info: api_structs::SummonerRanked::new(),
        debug_status: u16::from(status),
    };
    local_summoner.ranked_info = get_ranked_information(&local_summoner).await;
    return local_summoner;
}

// Return the ranked information of a given summoner
pub(crate) async fn get_ranked_information(local_summoner: &Summoner) -> api_structs::SummonerRanked{
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    let request_url: String = format!("https://{}.api.riotgames.com/lol/league/v4/entries/by-summoner/{}?api_key={}",local_summoner.region, local_summoner.summoner_info.id, riot_api);
    let mut resp = reqwest::get(request_url).await.expect("Failed to get a response");
    let resp = resp.text().await.expect("Could not parse");
    let local_summoner_ranked: api_structs::SummonerRanked = from_str(&resp).unwrap();
    local_summoner_ranked
}

// Get the specified number of match ids starting from a specified index.
pub(crate) async fn get_match_history(local_summoner: &Summoner, start_index: u8, count: u8) -> Vec<String> {
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    println!("{:#?}", local_summoner);
    let request_url: String = format!("https://{}.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids?start={}&count={}&api_key={}", local_summoner.routing_region, local_summoner.summoner_info.puuid, start_index, count, riot_api);
    let resp = reqwest::get(request_url).await.expect("Failed to get a response");
    let status :StatusCode = resp.status();
    println!("Match History Status: {}", status);
    if status == 200 {
        let resp= resp.text().await.expect("Could not parse");
        let match_vector: Vec<String> = from_str(&resp).unwrap();
        return match_vector;
    }
    Vec::new()
}