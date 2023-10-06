// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Match Information;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Match Information = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchInformation {
    pub metadata: Metadata,
    pub info: Info,
    #[serde(default)]
    pub participant_spells: Vec<Vec<Datum>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub game_creation: i64,
    pub game_duration: i64,
    pub game_end_timestamp: i64,
    pub game_id: i64,
    pub game_mode: String,
    pub game_name: String,
    pub game_start_timestamp: i64,
    pub game_type: String,
    pub game_version: String,
    pub map_id: i64,
    pub participants: Vec<Participant>,
    pub platform_id: String,
    pub queue_id: i64,
    pub queue_type: Option<String>,
    pub teams: Vec<Team>,
    pub tournament_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub all_in_pings: i64,
    pub assist_me_pings: i64,
    pub assists: i64,
    pub bait_pings: i64,
    pub baron_kills: i64,
    pub basic_pings: i64,
    pub bounty_level: i64,
    pub challenges: HashMap<String, f64>,
    pub champ_experience: i64,
    pub champ_level: i64,
    pub champion_id: i64,
    pub champion_name: String,
    pub champion_transform: i64,
    pub command_pings: i64,
    pub consumables_purchased: i64,
    pub damage_dealt_to_buildings: i64,
    pub damage_dealt_to_objectives: i64,
    pub damage_dealt_to_turrets: i64,
    pub damage_self_mitigated: i64,
    pub danger_pings: i64,
    pub deaths: i64,
    pub detector_wards_placed: i64,
    pub double_kills: i64,
    pub dragon_kills: i64,
    pub eligible_for_progression: bool,
    pub enemy_missing_pings: i64,
    pub enemy_vision_pings: i64,
    pub first_blood_assist: bool,
    pub first_blood_kill: bool,
    pub first_tower_assist: bool,
    pub first_tower_kill: bool,
    pub game_ended_in_early_surrender: bool,
    pub game_ended_in_surrender: bool,
    pub get_back_pings: i64,
    pub gold_earned: i64,
    pub gold_spent: i64,
    pub hold_pings: i64,
    pub individual_position: String,
    pub inhibitor_kills: i64,
    pub inhibitor_takedowns: i64,
    pub inhibitors_lost: i64,
    pub item0: i64,
    pub item1: i64,
    pub item2: i64,
    pub item3: i64,
    pub item4: i64,
    pub item5: i64,
    pub item6: i64,
    pub items_purchased: i64,
    pub killing_sprees: i64,
    pub kills: i64,
    pub lane: String,
    pub largest_critical_strike: i64,
    pub largest_killing_spree: i64,
    pub largest_multi_kill: i64,
    pub longest_time_spent_living: i64,
    pub magic_damage_dealt: i64,
    pub magic_damage_dealt_to_champions: i64,
    pub magic_damage_taken: i64,
    pub need_vision_pings: i64,
    pub neutral_minions_killed: i64,
    pub nexus_kills: i64,
    pub nexus_lost: i64,
    pub nexus_takedowns: i64,
    pub objectives_stolen: i64,
    pub objectives_stolen_assists: i64,
    pub on_my_way_pings: i64,
    pub participant_id: i64,
    pub penta_kills: i64,
    pub perks: Perks,
    pub physical_damage_dealt: i64,
    pub physical_damage_dealt_to_champions: i64,
    pub physical_damage_taken: i64,
    pub placement: Option<i64>,
    pub player_augment1: Option<i64>,
    pub player_augment2: Option<i64>,
    pub player_augment3: Option<i64>,
    pub player_augment4: Option<i64>,
    pub player_subteam_id: Option<i64>,
    pub profile_icon: i64,
    pub push_pings: i64,
    pub puuid: String,
    pub quadra_kills: i64,
    pub riot_id_name: String,
    pub riot_id_tagline: String,
    pub role: String,
    pub sight_wards_bought_in_game: i64,
    pub spell1_casts: i64,
    pub spell2_casts: i64,
    pub spell3_casts: i64,
    pub spell4_casts: i64,
    pub subteam_placement: Option<i64>,
    pub summoner1_casts: i64,
    pub summoner1_id: i64,
    pub summoner2_casts: i64,
    pub summoner2_id: i64,
    pub summoner_id: String,
    pub summoner_level: i64,
    pub summoner_name: String,
    pub team_early_surrendered: bool,
    pub team_id: i64,
    pub team_position: String,
    pub time_c_cing_others: i64,
    pub time_played: i64,
    pub total_ally_jungle_minions_killed: i64,
    pub total_damage_dealt: i64,
    pub total_damage_dealt_to_champions: i64,
    pub total_damage_shielded_on_teammates: i64,
    pub total_damage_taken: i64,
    pub total_enemy_jungle_minions_killed: i64,
    pub total_heal: i64,
    pub total_heals_on_teammates: i64,
    pub total_minions_killed: i64,
    #[serde(rename = "totalTimeCCDealt")]
    pub total_time_cc_dealt: i64,
    pub total_time_spent_dead: i64,
    pub total_units_healed: i64,
    pub triple_kills: i64,
    pub true_damage_dealt: i64,
    pub true_damage_dealt_to_champions: i64,
    pub true_damage_taken: i64,
    pub turret_kills: i64,
    pub turret_takedowns: i64,
    pub turrets_lost: i64,
    pub unreal_kills: i64,
    pub vision_cleared_pings: i64,
    pub vision_score: i64,
    pub vision_wards_bought_in_game: i64,
    pub wards_killed: i64,
    pub wards_placed: i64,
    pub win: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Perks {
    pub stat_perks: StatPerks,
    pub styles: Vec<Style>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatPerks {
    pub defense: i64,
    pub flex: i64,
    pub offense: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Style {
    pub description: Description,
    pub selections: Vec<Selection>,
    pub style: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Description {
    #[serde(rename = "primaryStyle")]
    PrimaryStyle,
    #[serde(rename = "subStyle")]
    SubStyle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Selection {
    pub perk: i64,
    pub var1: i64,
    pub var2: i64,
    pub var3: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub bans: Vec<Ban>,
    pub objectives: Objectives,
    pub team_id: i64,
    pub win: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ban {
    pub champion_id: i64,
    pub pick_turn: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Objectives {
    pub baron: Baron,
    pub champion: Baron,
    pub dragon: Baron,
    pub inhibitor: Baron,
    pub rift_herald: Baron,
    pub tower: Baron,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Baron {
    pub first: bool,
    pub kills: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub data_version: String,
    pub match_id: String,
    pub participants: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchType {
    pub queue_id: i64,
    pub map: String,
    pub description: Option<String>,
    pub notes: Option<String>,
}




// ------------- Summoners -----------------

pub type ParticipantSpells = Vec<Vec<Datum>>;

pub type SummonerSpells = Vec<SummonerSpell>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummonerSpell {
    #[serde(rename = "type")]
    pub summoner_spell_type: String,
    pub version: String,
    pub data: HashMap<String, Datum>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Datum {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tooltip: String,
    pub maxrank: i64,
    pub cooldown: Vec<f64>,
    pub cooldown_burn: String,
    pub cost: Vec<i64>,
    pub cost_burn: String,
    pub datavalues: Datavalues,
    pub effect: Vec<Option<Vec<f64>>>,
    pub effect_burn: Vec<Option<String>>,
    pub vars: Vec<Option<serde_json::Value>>,
    pub key: String,
    pub summoner_level: i64,
    pub modes: Vec<String>,
    pub cost_type: CostType,
    pub maxammo: String,
    pub range: Vec<i64>,
    pub range_burn: String,
    pub image: Image,
    pub resource: CostType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CostType {
    #[serde(rename = "&nbsp;")]
    Nbsp,
    #[serde(rename = "No Cost")]
    NoCost,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Datavalues {
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    pub full: String,
    pub sprite: Sprite,
    pub group: Group,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Group {
    Spell,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Sprite {
    #[serde(rename = "spell0.png")]
    Spell0Png,
}


