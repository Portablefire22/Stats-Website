// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Item;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Item = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "type")]
    pub item_type: Type,
    pub version: String,
    pub basic: Basic,
    pub data: HashMap<String, Datum>,
    pub groups: Vec<Group>,
    pub tree: Vec<Tree>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Basic {
    pub name: String,
    pub rune: Rune,
    pub gold: Gold,
    pub group: String,
    pub description: String,
    pub colloq: String,
    pub plaintext: String,
    pub consumed: bool,
    pub stacks: i64,
    pub depth: i64,
    pub consume_on_full: bool,
    pub from: Vec<Option<serde_json::Value>>,
    pub into: Vec<Option<serde_json::Value>>,
    pub special_recipe: i64,
    pub in_store: bool,
    pub hide_from_all: bool,
    pub required_champion: String,
    pub required_ally: String,
    pub stats: HashMap<String, i64>,
    pub tags: Vec<Option<serde_json::Value>>,
    pub maps: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gold {
    pub base: i64,
    pub total: i64,
    pub sell: i64,
    pub purchasable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rune {
    pub isrune: bool,
    pub tier: i64,
    #[serde(rename = "type")]
    pub rune_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datum {
    pub name: String,
    #[serde(default)]
    pub id: i64,
    pub description: String,
    pub colloq: String,
    pub plaintext: String,
    pub into: Option<Vec<String>>,
    pub image: Image,
    pub gold: Gold,
    pub tags: Vec<Tag>,
    pub maps: HashMap<String, bool>,
    pub stats: HashMap<String, f64>,
    pub effect: Option<Effect>,
    pub in_store: Option<bool>,
    pub from: Option<Vec<String>>,
    pub depth: Option<i64>,
    pub consumed: Option<bool>,
    pub stacks: Option<i64>,
    pub hide_from_all: Option<bool>,
    pub consume_on_full: Option<bool>,
    pub required_champion: Option<String>,
    pub required_ally: Option<RequiredAlly>,
    pub special_recipe: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Effect {
    pub effect1_amount: String,
    pub effect2_amount: Option<String>,
    pub effect3_amount: Option<String>,
    pub effect4_amount: Option<String>,
    pub effect5_amount: Option<String>,
    pub effect6_amount: Option<String>,
    pub effect7_amount: Option<String>,
    pub effect8_amount: Option<String>,
    pub effect9_amount: Option<String>,
    pub effect10_amount: Option<String>,
    pub effect11_amount: Option<String>,
    pub effect12_amount: Option<String>,
    pub effect13_amount: Option<String>,
    pub effect14_amount: Option<String>,
    pub effect15_amount: Option<String>,
    pub effect16_amount: Option<String>,
    pub effect17_amount: Option<String>,
    pub effect18_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub full: String,
    pub sprite: Sprite,
    pub group: Type,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Item,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sprite {
    #[serde(rename = "item0.png")]
    Item0Png,
    #[serde(rename = "item1.png")]
    Item1Png,
    #[serde(rename = "item2.png")]
    Item2Png,
    #[serde(rename = "item3.png")]
    Item3Png,
    #[serde(rename = "item4.png")]
    Item4Png,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequiredAlly {
    Ornn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Tag {
    #[serde(rename = "AbilityHaste")]
    AbilityHaste,
    Active,
    Armor,
    #[serde(rename = "ArmorPenetration")]
    ArmorPenetration,
    #[serde(rename = "AttackSpeed")]
    AttackSpeed,
    Aura,
    Boots,
    Consumable,
    #[serde(rename = "CooldownReduction")]
    CooldownReduction,
    #[serde(rename = "CriticalStrike")]
    CriticalStrike,
    Damage,
    #[serde(rename = "GoldPer")]
    GoldPer,
    Health,
    #[serde(rename = "HealthRegen")]
    HealthRegen,
    Jungle,
    Lane,
    #[serde(rename = "LifeSteal")]
    LifeSteal,
    #[serde(rename = "MagicPenetration")]
    MagicPenetration,
    #[serde(rename = "MagicResist")]
    MagicResist,
    Mana,
    #[serde(rename = "ManaRegen")]
    ManaRegen,
    #[serde(rename = "NonbootsMovement")]
    NonbootsMovement,
    #[serde(rename = "OnHit")]
    OnHit,
    Slow,
    #[serde(rename = "SpellBlock")]
    SpellBlock,
    #[serde(rename = "SpellDamage")]
    SpellDamage,
    #[serde(rename = "SpellVamp")]
    SpellVamp,
    Stealth,
    Tenacity,
    Trinket,
    Vision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    #[serde(rename = "MaxGroupOwnable")]
    pub max_group_ownable: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    pub header: String,
    pub tags: Vec<String>,
}
