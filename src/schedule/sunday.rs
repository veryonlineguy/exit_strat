use crate::schedule::utils;

pub fn sunday_schedule_str() -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    content.push("Slug Day".to_string());
    content.extend(utils::morning_str());
    content.push("Put away laptop".to_string());
    content.push("Set alarm for 5pm".to_string());
    content.extend(utils::evening_str());
    content.into_iter().map(|s| s.to_string()).collect()
}
