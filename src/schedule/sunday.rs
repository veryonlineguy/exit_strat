use crate::schedule::utils;

pub fn sunday_schedule_str() -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    content.extend(utils::morning_str());
    content.push("Brush teeth".to_string());
    content.push("Load towels".to_string());
    content.push("Check Ynab".to_string());
    content.push("duolingo".to_string());

    content.push("Audio book 40 min".to_string());
    content.push("Move towels".to_string());
    content.push("Brush teeth".to_string());
    content.push("Match socks".to_string());
    content.push("Load Laundry".to_string());
    content.push("Sweep bed + office + closet".to_string());
    content.push("Wipe down kitchen drawers".to_string());
    content.push("Windex mirror".to_string());
    content.push("Back up to external drive".to_string());
    content.push("Put away towels".to_string());
    content.push("Move Laundry".to_string());
    content.push("Audio book 40 min".to_string());
    let nails = utils::nails();
    content.extend(nails);
    content.push("grlfggt reflection".to_string());
    content.push("Put away laundry".to_string());
    content.push("Do dishes".to_string());
    content.extend(utils::journal());
    content.extend(utils::tea());
    content.extend(utils::goal_clothes_str());
    content.extend(utils::empty_ice_bin());
    content.push("Write tomorrow".to_string());
    content.push("Make meds".to_string());
    content.push("meds".to_string());
    content.extend(utils::evening_str());
    content
}
