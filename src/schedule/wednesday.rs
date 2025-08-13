use crate::schedule::utils;

pub fn wednesday_schedule_str() -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    content.extend(utils::morning_str());
    content.push("Brush teeth".to_string());
    content.extend(utils::tea());
    content.push("Clean up counter".to_string());
    content.push("Sweep hallway + bathroom".to_string());
    content.extend(utils::clean_silverware_drawer());
    content.push("Do dishes".to_string());
    content.extend(utils::evening_str());
    content
}
