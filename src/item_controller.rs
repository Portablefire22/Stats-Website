use std::fs;
use std::fs::File;
use std::io::Write;
use lazy_static::lazy_static;
use serde_json::Value;
use crate::item_structs::{Datum, Item};
use crate::match_structs::MatchInformation;

lazy_static! {
    #[derive(Debug)]
    //static ref ITEMS: Item = serde_json::from_str(include_str!("./static/datadragon/13.19.1/data/en_GB/item.json")).expect("Could not open the item json file");
    static ref ITEMS: Item = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/datadragon/13.19.1/data/en_GB/item.json"))).expect("Could not open the item json file");

}


// One must imagine Sisyphus happy


// Create a hashmap that can be quickly searched for the corresponding item.
// This is required because using the serde_json creates individual structs whose wait

pub async fn get_match_items(mut local_match: MatchInformation) -> MatchInformation {

    for participant in &local_match.info.participants {
        let mut items: Vec<Datum> = Vec::new();
        // Hehe >:3

        items.push(get_item_by_id(participant.item0).await);
        items.push(get_item_by_id(participant.item1).await);
        items.push(get_item_by_id(participant.item2).await);
        items.push(get_item_by_id(participant.item3).await);
        items.push(get_item_by_id(participant.item4).await);
        items.push(get_item_by_id(participant.item5).await);
        items.push(get_item_by_id(participant.item6).await);
        local_match.participant_info.participant_items.push(items);
    }
    return local_match
}

pub async fn get_item_by_id(item_id: i64) -> Datum {
    let items = &*ITEMS;
    if item_id == 0 {
        // Lets get a known good item
        let mut tmp: Datum = items.data.get("226695").expect(&*format!("Could not find an item by the given id: {}!", item_id)).clone();
        tmp.name = "NULL".parse().unwrap();
        tmp.id = 0;
        return tmp
    } else {
        let mut tmp = items.data.get(&item_id.to_string()).expect(&*format!("Could not find an item by the given id: {}!", item_id)).clone();
        tmp.id = item_id;
        return tmp
    }
}



// GOOD NEWS!
// Dont need to do this :D
pub async fn create_item_map() {
    fs::create_dir_all("debugfile/").expect("Could not create debug file dir");
    let mut test = File::create("debugfile/test").unwrap();
    File::write(&mut test, format!("{:#?}", &*ITEMS).as_ref());
    println!("{:#?}", &*ITEMS);
    panic!()
}