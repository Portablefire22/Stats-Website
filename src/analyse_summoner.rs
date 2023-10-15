use std::env;
use crate::api_structs::Summoner;
use crate::timeline_structs::Timeline;


pub async fn match_analysis(match_id: &str, focused_summoner: Summoner) -> Timeline {
    let timeline: Timeline = get_match_timeline(match_id, &focused_summoner).await;
    timeline
}

pub fn analyse_cs() -> () {

}

pub async fn get_match_timeline(match_id: &str, local_summoner: &Summoner) -> Timeline {
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    let request_url: String = format!("https://{}.api.riotgames.com/lol/match/v5/matches/{}/timeline?api_key={}",local_summoner.routing_region, match_id, riot_api);
    let mut resp = reqwest::get(request_url).await.expect("Failed to get a response");
    let resp = resp.text().await.expect("Could not parse");
    serde_json::from_str(&resp).unwrap()
}

