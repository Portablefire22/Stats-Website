// Handle getting game information


use std::env;
use crate::api_structs::Summoner;
use crate::{datadragon, match_structs};
use crate::match_structs::{MatchInformation, MatchType, Participant, SummonerSpell, SummonerSpells};
use std::fs;
use std::fs::File;
use std::io::Read;
use serde_json::to_string;

pub async fn get_match_details(local_summoner: &Summoner, game_id: String) -> MatchInformation{
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    let request_url: String = format!("https://{}.api.riotgames.com/lol/match/v5/matches/{}?api_key={}",local_summoner.routing_region, game_id, riot_api);
    let mut resp = reqwest::get(request_url).await.expect("Failed to get a response");
    let resp = resp.text().await.expect("Could not parse");
    let local_match: MatchInformation = serde_json::from_str(&resp).unwrap();
    let t = format!("{:#?}", local_match);
    fs::write("../test", t).expect("Unable to write to file");
    let participant: &Participant = get_participant_by_summoner(&local_summoner, &local_match).await;
    get_spell_by_id(participant.summoner1_id as i8).await;
    return local_match
}

pub async fn get_participant_by_summoner<'a>(local_summoner: &Summoner, local_match: &'a MatchInformation) -> &'a Participant {
    for participant in &local_match.info.participants {
        if participant.puuid == local_summoner.summoner_info.puuid {
            return participant;
        }
    }
    panic!("No participant found of the summoner's puuid, this shouldn't happen!")
}

pub async fn get_matches(local_summoner: &Summoner, match_ids: Vec<String>) -> Vec<MatchInformation> {
    let mut matches: Vec<MatchInformation> = Vec::new();
    for match_id in match_ids {
        let mut local_match = get_match_details(local_summoner, match_id).await;
        local_match.info.queue_type = Option::from(match local_match.info.queue_id {
            0 => "Custom",
            400 => "Normal Draft",
            420 => "Ranked Solo/Duo",
            430 => "Normal Blind",
            440 => "Ranked Flex",
            450 => "ARAM",
            700 => "Clash",
            720 => "ARAM Clash",
            830 | 840 | 850 => "Co-op Vs AI",
            1020 => "One For All",
            1300 | 1200 => "Nexus Blitz",
            1400 => "Ultimate Spellbook",
            1700 => "Arena",
            1900 => "URF",
            2000 | 2010 | 2020 => "Tutorial",
            900 => "ARURF",
            _ => "Unknown"
        }.parse::<String>().unwrap());
        matches.push(local_match);
    }
    let t = format!("{:#?}", matches);
    fs::write("test.test", t).expect("Unable to write vector to file");
    return matches;
}

// p.iter().find(|&x| x.id == 2).is_some()
pub async fn get_spell_by_id(summoner_spell_id: i8) -> bool {
    let mut summoner_file = File::open("static/datadragon/13.19.1/data/en_GB/summoner.json").expect("Could not open the summoner spell json file");
    let mut contents = String::new();
    summoner_file.read_to_string(&mut contents).expect("Could not read summoner file to string");
    let summoner_spells: SummonerSpells = serde_json::from_str(&*format!("{}", contents)).expect("Could not convert summoner spell to struct");
    println!("Summoner spell: {:#?}", summoner_spells[0].data.get(&summoner_spell_id.to_string()).expect("No summoner?"));
    //assert!(summoner_spells.iter().find(|&x| x.data.get(&summoner_spell_id.to_string()).expect("No summoner?") == 2));
    false

}

