use std::env;
use crate::api_structs::Summoner;
use crate::timeline_structs::{Frame, Participant, Timeline};


pub async fn match_analysis(match_id: &str, focused_summoner: Summoner) -> Timeline {
    let timeline: Timeline = get_match_timeline(match_id, &focused_summoner).await;
    let mut focused_id = 64;
    for participant in &timeline.info.participants {
        if participant.puuid == focused_summoner.summoner_info.puuid {
            focused_id = participant.participant_id;
            break;
        }
    }
    let frames: Vec<Frame> = get_frames(&timeline);
    analyse_cs(&frames, &focused_summoner, &focused_id);
    timeline
}

// Not doing any manipulation to the frames so I can just use their reference and not worry about moving them around
pub fn analyse_cs(frames: &Vec<Frame>, focused_summoner: &Summoner, focused_id: &i64) -> () {
    // Each frame is a minute of game time
    let mut cs_per_minute: Vec<f64> = Vec::new();
    for (i, frame) in frames.into_iter().enumerate() {
        // Participant frames start from one
        let participant_frame = frame.participant_frames.get(&*format!("{}",focused_id)).expect(&*format!("Could not find a participant by the id: {}", focused_id));
        let minions = participant_frame.minions_killed;
        if i > 0 {
            let cs_min: f64 = minions as f64 / i as f64;
            println!("{}, {:.1}/m", minions, cs_min);

            cs_per_minute.push(cs_min);
        }
    }
    dbg!(cs_per_minute);
}

pub fn get_frames(timeline: &Timeline) -> Vec<Frame> {
    return timeline.info.frames.clone();
}
pub async fn get_match_timeline(match_id: &str, local_summoner: &Summoner) -> Timeline {
    let riot_api: String = (env::var("RIOT_API").unwrap()).replace('"', "");
    let request_url: String = format!("https://{}.api.riotgames.com/lol/match/v5/matches/{}/timeline?api_key={}",local_summoner.routing_region, match_id, riot_api);
    let mut resp = reqwest::get(request_url).await.expect("Failed to get a response");
    let resp = resp.text().await.expect("Could not parse");

    serde_json::from_str(&resp).unwrap()
}

