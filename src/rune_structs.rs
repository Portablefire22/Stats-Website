use serde::{Serialize, Deserialize};
use std::collections::HashMap;
pub type Rune = Vec<RuneElement>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuneElement { // The rune tree it's from e.g. precision, inspiration
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub slots: Vec<Slot>,
    #[serde(default)]
    pub index: HashMap<i64, i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slot {
    pub runes: Vec<RuneClass>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuneClass { // The actual rune
    #[serde(default)]
    pub rune_tree: String,
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub short_desc: String,
    pub long_desc: String,
}

/*
    Slots {
        Keystone
        Row 1
        Row 2
        Row 3
 */