use crate::schedule::utils;

pub fn saturday_schedule_str() -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    content.push("Slug Day".to_string());


    content.into_iter().map(|s| s.to_string()).collect()
}
