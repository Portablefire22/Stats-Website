//fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Timeline = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub metadata: Metadata,
    pub info: Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub frame_interval: i64,
    pub frames: Vec<Frame>,
    pub game_id: i64,
    pub participants: Vec<Participant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub events: Vec<Event>,
    pub participant_frames: HashMap<String, ParticipantFrame>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub real_timestamp: Option<i64>,
    pub timestamp: i64,
    #[serde(rename = "type")]
    pub event_type: String,
    pub level: Option<i64>,
    pub participant_id: Option<i64>,
    pub level_up_type: Option<LevelUpType>,
    pub skill_slot: Option<i64>,
    pub item_id: Option<i64>,
    pub assisting_participant_ids: Option<Vec<i64>>,
    pub bounty: Option<i64>,
    pub kill_streak_length: Option<i64>,
    pub killer_id: Option<i64>,
    pub position: Option<Position>,
    pub shutdown_bounty: Option<i64>,
    pub victim_damage_dealt: Option<Vec<VictimDamage>>,
    pub victim_damage_received: Option<Vec<VictimDamage>>,
    pub victim_id: Option<i64>,
    pub kill_type: Option<KillType>,
    pub multi_kill_length: Option<i64>,
    pub after_id: Option<i64>,
    pub before_id: Option<i64>,
    pub gold_gain: Option<i64>,
    pub creator_id: Option<i64>,
    pub ward_type: Option<String>,
    pub building_type: Option<String>,
    pub lane_type: Option<String>,
    pub team_id: Option<i64>,
    pub tower_type: Option<String>,
    pub game_id: Option<i64>,
    pub winning_team: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KillType {
    #[serde(rename = "KILL_ACE")]
    KillAce,
    #[serde(rename = "KILL_FIRST_BLOOD")]
    KillFirstBlood,
    #[serde(rename = "KILL_MULTI")]
    KillMulti,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LevelUpType {
    #[serde(rename = "NORMAL")]
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VictimDamage {
    pub basic: bool,
    pub magic_damage: i64,
    pub name: String,
    pub participant_id: i64,
    pub physical_damage: i64,
    pub spell_name: String,
    pub spell_slot: i64,
    pub true_damage: i64,
    #[serde(rename = "type")]
    pub victim_damage_type: VictimDamageDealtType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VictimDamageDealtType {
    #[serde(rename = "MINION")]
    Minion,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "TOWER")]
    Tower,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantFrame {
    pub champion_stats: HashMap<String, i64>,
    pub current_gold: i64,
    pub damage_stats: HashMap<String, i64>,
    pub gold_per_second: i64,
    pub jungle_minions_killed: i64,
    pub level: i64,
    pub minions_killed: i64,
    pub participant_id: i64,
    pub position: Position,
    pub time_enemy_spent_controlled: i64,
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub participant_id: i64,
    pub puuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub data_version: String,
    pub match_id: String,
    pub participants: Vec<String>,
}