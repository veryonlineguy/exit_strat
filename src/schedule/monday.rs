use crate::schedule::utils;

// Schedule content generators
pub fn monday_schedule_str() -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    content.extend(utils::morning_str());
    content.extend(utils::change_razor_str());
    content.extend(utils::shave_head_str());
    content.push("Sweep hallway + bathroom".to_string());
    content.push("Swifter".to_string());
    content.extend(utils::journal());
    content.push("Clean up counter".to_string());
    content.push("Brush teeth".to_string());
    content.push("5 min linkedin".to_string());
    content.push("Clean bathroom".to_string());
    content.push("Do dishes".to_string());
    content.push("gratitude for yesterday".to_string());
    content.push("meds".to_string());
    content.extend(utils::evening_str());
    content
}
