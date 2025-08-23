use crate::util::DATA_DIR;
use rand;
use rand::prelude::IndexedRandom;
use std::fs::read_to_string;

fn read_affirmations() -> String {
    let lines = read_to_string(DATA_DIR.to_owned() + "/growth").unwrap();
    let strings: Vec<String> = lines.lines().map(String::from).collect();

    let mut rng = rand::rng();

    let mut ret = "".to_string();

    let random_beliefs: Vec<String> = strings.choose_multiple(&mut rng, 5).cloned().collect();

    for belief in random_beliefs {
        ret += &("\n- ".to_owned() + &belief);
    }

    ret += "\n";
    ret.to_string()
}

fn read_suggestions() -> String {
    let lines = read_to_string(DATA_DIR.to_owned() + "/suggestions").unwrap();
    let strings: Vec<String> = lines.lines().map(String::from).collect();

    let mut rng = rand::rng();

    let mut ret = "".to_string();

    let random_beliefs: Vec<String> = strings.choose_multiple(&mut rng, 5).cloned().collect();

    for belief in random_beliefs {
        ret += &("\n".to_owned() + &belief);
    }

    ret += "\n";
    ret.to_string()
}

pub fn get_affirmations() -> String {
    let data: &str =
        "- I don't think being trans is bad\n- I'm frugal\n- Consider what the other person needs";

    let affirmations: String = read_affirmations();

    let data = data.to_owned() + &affirmations;

    let data = data + &read_suggestions();
    data
}
