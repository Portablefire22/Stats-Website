use std::env;
use reqwest::StatusCode;
use crate::api_structs;
use crate::api_structs::Summoner;

// Get the summoner information
pub(crate) async fn get_summoner(region: &str, username: &str) -> api_structs::Summoner {
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    let safe_username: String = username.replace(" ", "%20");
    let request_url: String = format!("https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}?api_key={}",region, safe_username, &riot_api);
    let mut resp = reqwest::get(request_url).await.expect("Failed to get a response");
    let status :StatusCode = resp.status();
    println!("Status: {}", status);
    if status == 200 {
        let resp: String = resp.text().await.expect("Could not parse");
        let mut local_summoner_info: api_structs::SummonerInfo = serde_json::from_str(&resp).unwrap();
        let mut local_summoner = api_structs::Summoner {
            summoner_info: local_summoner_info,
            region: region.parse().unwrap(),
            ranked_info: api_structs::SummonerRanked::new(),
            debug_status: u16::from(status),
        };
        local_summoner.ranked_info = get_ranked_information(&local_summoner).await;
        return local_summoner;
    }
    let mut default_summoner: api_structs::Summoner = api_structs::Summoner::default();
    default_summoner.debug_status = u16::from(status);
    return default_summoner;
}



// Return the ranked information of a given summoner
pub(crate) async fn get_ranked_information(local_summoner: &api_structs::Summoner) -> api_structs::SummonerRanked{
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    let request_url: String = format!("https://{}.api.riotgames.com/lol/league/v4/entries/by-summoner/{}?api_key={}", local_summoner.region, local_summoner.summoner_info.id, riot_api);
    let resp = reqwest::get(request_url).await.expect("Failed to get a response").text().await.expect("Could not parse");
    let local_summoner_ranked: api_structs::SummonerRanked = serde_json::from_str(&resp).unwrap();
    local_summoner_ranked
}


