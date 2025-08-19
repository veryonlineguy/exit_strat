use crate::schedule::utils;

pub fn friday_schedule_str() -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    content.extend(utils::morning_str());
    content.push("Brush Teeth".to_string());
    content.push("Check Ynab".to_string());
    content.push("Audio book 40 min".to_string());
    content.extend(utils::gratitude());
    content.push("Clean up counter".to_string());
    content.push("Sweep hallway + bathroom".to_string());
    content.push("Swifter".to_string());
    content.push("Take out trash".to_string());
    content.push("Weekly Reflection".to_string());
    content.extend(utils::evening_str());
    content
}
