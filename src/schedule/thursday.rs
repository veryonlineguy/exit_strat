use crate::schedule::utils;

pub fn thursday_schedule_str() -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    content.extend(utils::morning_str());
    content.push("Cook".to_string());
    content.push("Match socks".to_string());
    content.push("Load Laundry".to_string());
    content.push("Sweep bed + office + closet".to_string());
    content.push("Swifter".to_string());
    content.push("Throw something out".to_string());
    content.push("Move Laundry".to_string());
    content.push("Put away laundry".to_string());
    content.push("Do dishes".to_string());
    content.extend(utils::evening_str());
    content
}
