use crate::schedule::utils;

pub fn wednesday_schedule_str() -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    content.extend(utils::morning_str());
    content.push("Brush teeth".to_string());
    content.push("Check Ynab".to_string());
    content.push("10 min dbt".to_string());
    content.push("Cook".to_string());
    content.extend(utils::tea());
    content.push("Audio book 40 min".to_string());
    content.push("Clean up counter".to_string());
    content.push("Sweep hallway + bathroom".to_string());
    content.extend(utils::clean_silverware_drawer());
    content.extend(utils::journal());
    content.push("Whey Protein".to_string());
    content.push("Brush teeth".to_string());
    content.extend(utils::tea());
    content.push("Do dishes".to_string());
    content.push("Write Tomorrow".to_string());
    content.push("gratitude for yesterday".to_string());
    content.push("meds".to_string());
    content.extend(utils::evening_str());
    content
}
