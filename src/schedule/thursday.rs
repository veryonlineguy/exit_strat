use crate::schedule::utils;

pub fn thursday_schedule_str() -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    content.extend(utils::morning_str());
    content.push("Blood pressure on sheet".to_string());
    content.push("Brush teeth".to_string());
    content.push("Cook".to_string());
    content.extend(utils::tea());
    content.push("Audio book 40 min".to_string());
    content.push("Clean up counter".to_string());
    content.push("Check Ynab".to_string());
    content.push("Shave legs".to_string());
    content.push("Match socks".to_string());
    content.push("Load Laundry".to_string());
    content.push("Sweep bed + office + closet".to_string());
    content.push("Swifter".to_string());
    content.extend(utils::journal());
    content.push("10 min dbt".to_string());
    content.push("Take off nail polish".to_string());
    content.push("Sec video".to_string());
    content.extend(utils::buy_stuff_str());
    content.push("Growth vid".to_string());
    content.push("Throw something ())out".to_string());
    content.push("Move Laundry".to_string());
    content.push("10 min bsky".to_string());
    content.push("Put away laundry".to_string());
    content.push("Do dishes".to_string());
    content.push("Mid day gratitude:".to_string());
    content.extend(utils::tea());
    content.push("Write tomorrow".to_string());
    content.push("Floss".to_string());
    content.push("Brush teeth".to_string());
    content.push("Grat yesteday".to_string());
    content.push("meds".to_string());
    content.extend(utils::evening_str());
    content.extend(utils::makeup_str());
    content
}
