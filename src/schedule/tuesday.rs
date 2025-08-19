use crate::schedule::utils;

pub fn tuesday_schedule_str() -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    content.extend(utils::morning_str());
    content.push("Blood pressure on sheet".to_string());
    content.push("Brush teeth".to_string());
    content.push("Cook".to_string());
    content.extend(utils::tea());
    content.push("Audio book 40 min".to_string());
    content.push("Clean up counter".to_string());
    content.push("Match socks".to_string());

    content.push("Load Laundry".to_string());
    content.push("Sweep bed + office + closet".to_string());
    content.push("Swifter".to_string());
    content.extend(utils::clean_washer_str());
    content.push("Clean Closet".to_string());

    content.push("10 min dbt".to_string());
    content.extend(utils::journal());
    content.push("Move Laundry".to_string());
    content.push("Wash HRM".to_string());
    content.extend(utils::gratitude());
    content.push("Brush teeth".to_string());

    content.push("Put away laundry".to_string());
    content.extend(utils::tea());
    content.push("Do Dishes".to_string());
    content.push("Write tomorrow".to_string());
    content.extend(utils::vid_str());
    content.push("zazen".to_string());
    content.push("gratitude for yesterday".to_string());
    content.push("meds".to_string());
    content.extend(utils::evening_str());
    content
}
