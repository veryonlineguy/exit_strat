use crate::schedule::utils;

pub fn saturday_schedule_str() -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    let morning_str = utils::morning_str();
    content.extend(morning_str);
    content.push("Cook".to_string());
    content.push("Audio book 40 min".to_string());
    content.push("Clean up counter".to_string());
    content.push("Brush teeth".to_string());
    content.push("Clean closet".to_string());
    content.push("Match socks".to_string());
    content.push("Load Laundry".to_string());
    content.push("Sweep bed + office + closet".to_string());
    content.extend(utils::clean_silverware_drawer());
    content.push("Review Allowed Buy".to_string());
    content.push("Clean bathroom".to_string());
    content.push("Do dishes".to_string());
    content.push("Move laundry".to_string());
    content.push("Put away laundry".to_string());
    content.push("gratitude for yesterday".to_string());
    content.push("meds".to_string());
    let evening_str = utils::evening_str();
    content.extend(evening_str);

    content.into_iter().map(|s| s.to_string()).collect()
}
