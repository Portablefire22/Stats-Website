// Handle getting game information


use std::collections::HashMap;
use std::env;
use crate::api_structs::Summoner;
use crate::{datadragon, match_structs};
use crate::match_structs::{Datum, MatchInformation, MatchType, Participant, ParticipantSpells, SummonerSpell, SummonerSpells};

use std::fs;
use std::fs::File;
use std::future::join;
use std::hint::unreachable_unchecked;
use std::io::{BufReader, Read, Write};
use rocket::futures::future;
use serde_json::to_string;
use syn::ReturnType::Default;
use crate::item_controller::{create_item_map, get_item_by_id, get_match_items};
use crate::rune_structs::{RuneElement, RuneClass, Slot, Rune};


pub async fn get_match_details(local_summoner: &Summoner, game_id: String) -> MatchInformation{
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    let request_url: String = format!("https://{}.api.riotgames.com/lol/match/v5/matches/{}?api_key={}",local_summoner.routing_region, game_id, riot_api);
    let mut resp = reqwest::get(request_url).await.expect("Failed to get a response");
    let resp = resp.text().await.expect("Could not parse");
    let mut local_match: MatchInformation = serde_json::from_str(&resp).unwrap();
    local_match = get_match_spells(local_match).await;
    local_match = get_match_runes(local_match).await;
    local_match = get_match_items(local_match).await;
    let t = format!("{:#?}", local_match);
    fs::write("../test", t).expect("Unable to write to file");
    return local_match
}

pub async fn get_match_spells(mut local_match: MatchInformation) -> MatchInformation {
    for participants in &local_match.info.participants {
        local_match.participant_info.participant_spells.push(vec![get_spell_by_id(participants.summoner1_id).await, get_spell_by_id(participants.summoner2_id).await]);
    }
    local_match
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
        println!("{:#?}", &match_id);
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
pub async fn get_spell_by_id(summoner_spell_id: i64) -> Datum {
    fs::create_dir_all("hashmaps/").expect("Could not create hashmap dir");
    let mut summoner_file = File::open("static/datadragon/13.19.1/data/en_GB/summoner.json").expect("Could not open the summoner spell json file");
    let mut contents = String::new();
    summoner_file.read_to_string(&mut contents).expect("Could not read summoner file to string");
    let summoner_spells: SummonerSpells = serde_json::from_str(&*format!("{}", contents)).expect("Could not convert summoner spell to struct");
    let mut summoner_map;
    let hashmap_file = File::open("hashmaps/spell_ids.json");
    if hashmap_file.is_err(){
        summoner_map = HashMap::new();
        for (key, tmp_spell) in &summoner_spells[0].data {
            summoner_map.insert(&tmp_spell.key, &tmp_spell.id);
        }
        std::mem::drop(hashmap_file);
        let hashmap_file = File::create("hashmaps/spell_ids.json");
        serde_json::to_writer(hashmap_file.unwrap(), &summoner_map).expect("Hashmap file for summoner spells does not exist!");
    }
    let mut hashmap_file = File::open("hashmaps/spell_ids.json").expect("Hashmap file does not exist, this shouldn't be happening!");
    let reader = BufReader::new(hashmap_file);
    let mut hash_json: HashMap<String, String> = serde_json::from_reader(reader).unwrap();
    //let hash_json: HashMap<&String, &String> = serde_json::from_str().expect("Summoner json not formatted!");
    let temp_key = hash_json.get(&summoner_spell_id.to_string()).unwrap();
    summoner_spells[0].data.get(&String::from(temp_key)).unwrap().clone()
}

pub async fn get_match_runes(mut local_match: MatchInformation) -> MatchInformation {
    //get_rune_by_id(8124).await;
    for (i,participant) in local_match.info.participants.iter().enumerate() {
        local_match.participant_info.participant_runes.push(build_runes(participant).await);
        //println!("{:#?}", local_match.participant_info.participant_runes);

    }

    return local_match
}

pub async fn build_runes(participant: &Participant) -> RuneElement{
    let (tree_map, rune_map) = create_rune_map().await;
    // To build the rune we must first have the rune tree

    // Runes are a vector so
    // [Main Runes, Sub runes]
    // Main runes being from the main tree
    // Subrunes being the two secondary

    // The style in i64 is the rune tree
    println!("---------{}:{}---------", participant.participant_id, participant.summoner_name);
    let mut rune_construct = tree_map.get(&participant.perks.styles[0].style).expect("Cant find a rune tree with the specified id! (This shouldn't be possible! Is datadragon up to date?)").clone();
    rune_construct.slots = vec!();
    for (i, class) in participant.perks.styles.iter().enumerate() {
        rune_construct.slots.push(Slot{ runes: vec![] });
        for rune in &class.selections {
            let test = get_rune_by_id(rune.perk, &rune_map).await;
            rune_construct.slots[i].runes.push(test.clone());
            //println!("{:#?}", test);
            //println!("========");
        }
    }

    return rune_construct
}


// Looks kinda useless but idk though, might be expanded in the future
pub async fn get_rune_by_id(rune_id: i64, rune_map: &HashMap<i64, RuneClass>) -> RuneClass {
    let rune = rune_map.get(&rune_id).expect(&*format!("Failed to get a rune of id: {}", rune_id)).clone();
    return rune;
}


pub async fn create_rune_map() -> (HashMap<i64, RuneElement>,HashMap<i64, RuneClass>) {
    fs::create_dir_all("hashmaps/").expect("Could not createhashmap dir");
    let mut rune_file = File::open("static/datadragon/13.19.1/data/en_GB/runesReforged.json").expect("Could not open the rune json file");
    let mut contents = String::new();
    rune_file.read_to_string(&mut contents).expect("Could not read rune file to string");
    let runes: Rune = serde_json::from_str(&*format!("{}", contents)).expect("Could not convert rune to struct");
    // First check if the map exists
    let rune_hashmap_file = File::open("hashmaps/runes.json");
    let subrune_hashmap_file = File::open("hashmaps/subrunes.json");
    let mut rune_map: HashMap<i64, RuneElement>;
    let mut subrune_map: HashMap<i64, RuneClass>;
    if rune_hashmap_file.is_err() || subrune_hashmap_file.is_err() {
        rune_map = HashMap::new();
        subrune_map = HashMap::new();
        for rune in runes {
            println!("{:?}", rune);
            rune_map.insert(rune.id, rune.clone());
            for slot in rune.slots{
                for mut subrune in slot.runes {
                    subrune.rune_tree = rune.key.clone();
                    subrune_map.insert(subrune.id, subrune);
                }
            }
        }
        std::mem::drop(rune_hashmap_file);
        std::mem::drop(subrune_hashmap_file);
        let hashmap_file = File::create("hashmaps/runes.json");
        let sub_hashmap_file = File::create("hashmaps/subrunes.json");
        serde_json::to_writer(hashmap_file.unwrap(), &rune_map).expect("Hashmap file for summoner spells does not exist!");
        serde_json::to_writer(sub_hashmap_file.unwrap(), &subrune_map).expect("Hashmap file for summoner spells does not exist!");
    }
    let mut rune_hashmap_file = File::open("hashmaps/runes.json").expect("Hashmap file does not exist, this shouldn't be happening!");
    let reader = BufReader::new(rune_hashmap_file);
    let mut runepath_json: HashMap<i64, RuneElement> = serde_json::from_reader(reader).unwrap();
    let mut subrune_hashmap_file = File::open("hashmaps/subrunes.json").expect("Hashmap file does not exist, this shouldn't be happening!");
    let sub_reader = BufReader::new(subrune_hashmap_file);
    let mut rune_info_json: HashMap<i64, RuneClass> = serde_json::from_reader(sub_reader).unwrap();
    (runepath_json, rune_info_json)

}