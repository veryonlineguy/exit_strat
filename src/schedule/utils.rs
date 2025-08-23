use crate::affirmations;
use crate::lift;
use crate::menu::get_menu;
use crate::schedule::friday;
use crate::schedule::monday;
use crate::schedule::saturday;
use crate::schedule::sunday;
use crate::schedule::thursday;
use crate::schedule::tuesday;
use crate::schedule::wednesday;
use crate::util::VAULT_DIR;
use chrono::Duration;
use chrono::{DateTime, Datelike, Days, Local, NaiveDate, Weekday};
use rand::Rng;
use rand::seq::IndexedRandom;
use std::fs::File;
use std::io::Write;

fn get_schedule_content(day: Weekday) -> String {
    match day {
        Weekday::Mon => format_day(monday::monday_schedule_str()),
        Weekday::Tue => format_day(tuesday::tuesday_schedule_str()),
        Weekday::Wed => format_day(wednesday::wednesday_schedule_str()),
        Weekday::Thu => format_day(thursday::thursday_schedule_str()),
        Weekday::Fri => format_day(friday::friday_schedule_str()),
        Weekday::Sat => format_day(saturday::saturday_schedule_str()),
        Weekday::Sun => format_day(sunday::sunday_schedule_str()),
    }
}

pub fn days_since() -> i64 {
    let earlier = NaiveDate::from_ymd_opt(2025, 7, 31).unwrap();
    // Get today's date in UTC
    let today = chrono::Local::now();
    let today = NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap();
    // Compute the difference as a Duration
    let diff: Duration = today.signed_duration_since(earlier);
    // Return the number of days (truncating any fractional days)
    diff.num_days()
}

pub fn write_tomorrow_to_vault() {
    let local: DateTime<Local> = Local::now();
    let tomorrow: NaiveDate =
        NaiveDate::from_ymd_opt(local.year(), local.month(), local.day()).unwrap();
    let tomorrow = tomorrow + Days::new(1);
    let day = tomorrow.weekday();

    let content = get_schedule_content(day);
    let date_str = tomorrow.format("%Y-%m-%d").to_string();
    let filename = format!("{}/{}.md", VAULT_DIR, date_str);

    let mut full_content = String::new();
    full_content.push_str("# The way out is in\n\n");
    let rules = "## Rules\n- No Uber Eats (".to_owned()
        + &days_since().to_string()
        + "/100)\nTil out of debt and under 200lb\n\n";
    full_content.push_str(&rules);

    full_content.push_str(&format!("## Day {}\n\n", day));
    full_content.push_str(&content);

    full_content.push_str("\n# Menu\n");
    let menu = get_menu();

    full_content.push_str(&menu);

    let lift = lift::get_lifts();
    full_content.push_str(&lift);

    full_content.push_str("\n# Daily reflection\n");
    full_content.push_str("\n## One word to describe the day\n\n");
    full_content.push_str("\n## Wins\n- \n\n");
    full_content.push_str("# Affirmations\n\n");

    full_content.push_str(&affirmations::get_affirmations());

    match File::create(&filename) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(full_content.as_bytes()) {
                eprintln!("Error writing to file {}: {}", filename, e);
            }
        }
        Err(e) => {
            eprintln!("Error creating file {}: {}", filename, e);
        }
    }
}

fn format_day(day: Vec<String>) -> String {
    let mut count = 1;
    let mut result = String::new();
    for task in &day {
        let todo = format!("{}. [ ] {}\n", count, task);
        result.push_str(&todo);
        count += 1;
    }
    result
}

pub fn print_monday_schedule() {
    print!("{}", format_day(monday::monday_schedule_str()));
}

pub fn print_tuesday_schedule() {
    print!("{}", format_day(tuesday::tuesday_schedule_str()));
}

pub fn print_wednesday_schedule() {
    print!("{}", format_day(wednesday::wednesday_schedule_str()));
}

pub fn print_thursday_schedule() {
    print!("{}", format_day(thursday::thursday_schedule_str()));
}

pub fn print_friday_schedule() {
    print!("{}", format_day(friday::friday_schedule_str()));
}

pub fn print_saturday_schedule() {
    print!("{}", format_day(saturday::saturday_schedule_str()));
}

pub fn print_sunday_schedule() {
    print!("{}", format_day(sunday::sunday_schedule_str()));
}

// String versions of utility functions that normally print
pub fn shave_head_str() -> Vec<String> {
    if rand::rng().random_range(0..3) == 0 {
        vec!["Watch ecology of horrors talk".to_string()]
    } else {
        vec![]
    }
}

pub fn clean_washer_str() -> Vec<String> {
    let week = current_week_number();

    if week % 4 == 0 {
        vec!["Clean washer 1 cup vinegar".to_string()]
    } else {
        vec![]
    }
}

pub fn journal() -> Vec<String> {
    vec!["Two page of thoughts".to_string()]
}

pub fn vid_str() -> Vec<String> {
    let options = [
        "Growth",
        "Slop",
        "Slop",
        "Slop",
        "Code",
        "Code",
        "trans",
        "Spirituality",
        "Makeup",
        "We go gym",
        "Math",
        "Notes",
    ];

    let choice = options.choose(&mut rand::rng()).unwrap().to_string();
    vec![format!("{} vid", choice)]
}

pub fn morning_str() -> Vec<String> {
    let mut result = Vec::new();
    let local: DateTime<Local> = Local::now();
    let tomorrow: NaiveDate =
        NaiveDate::from_ymd_opt(local.year(), local.month(), local.day()).unwrap();
    let tomorrow = tomorrow + Days::new(1);
    let day = tomorrow.weekday();
    result.push("Wash Retainer");
    result.push("Weigh");
    result.push("Write weight in field notes");
    result.push("5 min reading");
    result.push("Do DIshes");
    result.push("Cook");

    result.push("Daily intention Post It");
    result.push("Write suggestions down");
    result.push("Check Obsidian cal");
    result.push("Charge watch");
    result.push("Write everything you're stresed about");
    match day {
        Weekday::Mon | Weekday::Wed | Weekday::Fri => result.push("Wipe out sink"),
        _ => (),
    }

    match day {
        Weekday::Tue | Weekday::Thu => result.push("Take out Trash"),
        _ => (),
    }
    result.push("Anki");
    result.push("Work Anki");
    result.push("Big Game Anki");

    match day {
        Weekday::Mon | Weekday::Wed | Weekday::Fri => result.push("Shave face"),
        _ => (),
    }

    let shot_str = get_shot_leg();

    match day {
        Weekday::Mon => {
            result.push(&shot_str);
        }
        _ => (),
    }
    match day {
        Weekday::Mon | Weekday::Wed | Weekday::Fri => result.push("Keal vid"),
        _ => (),
    }
    result.into_iter().map(|s| s.to_string()).collect()
}

pub fn evening_str() -> Vec<String> {
    let mut result = Vec::new();

    result.push("Take meds".to_string());
    result.push("Floss".to_string());
    result.push("Brush".to_string());
    result.push("Retainer".to_string());
    result.push("Log weight".to_string());
    result.push("Read through tomorrow".to_string());
    result.push("Reflection".to_string());

    result
}

pub fn tea() -> Vec<String> {
    let options = ["Orange Creamsicle", "Pink Lemon Ginger", "Turmeric Ginger"];
    let tea = options.choose(&mut rand::rng()).unwrap();
    let _tea = format!("Cup of {}", tea).to_string();
    vec![]
}

pub fn goal_clothes_str() -> Vec<String> {
    let weeks = get_current_week();

    if weeks % 6 == 0 {
        vec!["Try on goal clothes".to_string()]
    } else {
        vec![]
    }
}

pub fn clean_silverware_drawer() -> Vec<String> {
    let weeks = get_current_week();

    if weeks % 4 == 0 {
        vec!["Clean silverware drawer".to_string()]
    } else {
        vec![]
    }
}

pub fn gratitude() -> Vec<String> {
    vec!["Write on thing your grateful for".to_string()]
}

pub fn makeup_str() -> Vec<String> {
    let lip_options = ["Kush Lip Oil", "Summer Fridays", "Peach Glaze"];
    let scents = ["Glossier scent", "Britney Spears scent"];

    let mut result: Vec<String> = Vec::new();
    result.push("Mascara".to_string());
    result.push("Eye Liner".to_string());
    let lip_option = lip_options.choose(&mut rand::rng()).unwrap();
    result.push(format!("lip: {}", lip_option));
    let scent = scents.choose(&mut rand::rng()).unwrap();
    result.push(format!("{}", scent));
    result
}

pub fn nails() -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    let options = ["Crustacean", "Hula hoop"];

    content.push(options.choose(&mut rand::rng()).unwrap().to_string());
    content.push("File nails".to_string());
    content.push("color".to_string());
    content.push("10 min wait".to_string());
    content.push("color".to_string());
    content.push("10 min wait".to_string());
    content.push("color".to_string());
    content.push("20 min wait".to_string());
    content.push("top coat".to_string());
    content.push("wait 10 minutes".to_string());

    let _content = content;

    vec![]
}

pub fn change_razor_str() -> Vec<String> {
    let week = current_week_number();

    if week % 4 == 0 {
        vec!["Change leg razor".to_string()]
    } else {
        vec![]
    }
}

pub fn buy_stuff_str() -> Vec<String> {
    let week = get_current_week();
    if week % 4 == 1 {
        vec!["BUY! BUY! BUY!".to_string()]
    } else {
        vec![]
    }
}

pub fn empty_ice_bin() -> Vec<String> {
    let week = get_current_week();
    if week % 12 == 1 {
        vec!["empty ice bin".to_string()]
    } else {
        vec![]
    }
}

pub fn current_week_number() -> u32 {
    let today = Local::now().date_naive();
    today.iso_week().week()
}

fn get_shot_leg() -> String {
    let week = current_week_number();

    let shot = match week % 2 {
        0 => "Do shot L".to_string(),
        _ => "Do shot R".to_string(),
    };
    shot
}

pub fn _movie() -> Vec<String> {
    let hulu_movies = vec![
        "Maxxxine",
        "Anora",
        "Paris is burning",
        "Showgirls",
        "Twin peaks fire walk with me",
        "Logan",
        "The truth Vs. Alex jones",
        "Implosion",
        "Beau is afraid",
        "Barbie",
        "Ford v F4errari",
        "Borat",
        "The death of stalin",
        "Idiocarcy",
        "The martian",
        "COffee and cigarttes",
        "Hurricane Bianca",
        "Blue Jean",
        "Tangerine",
        "Adam",
        "Black swan",
        "Dirty Dancing",
        "Joker",
        "Wall Street",
        "Free SOlo",
        "Life of pi",
        "Working Girl",
        "Venom the last dance",
        "Life aquatic",
        "Once upon a time in hollywood",
        "Barbie",
        "The godfather",
        "The devil wears prada",
        "Lady bird",
        "SPirited away",
        "Reservoir dogs",
        "Once upon a time in hollywood",
        "The french dispactch",
        "district 9",
    ];

    let netflix_movies = vec![
        "American manhunt osam bin laden",
        "Titan",
        "Train wreck ppo cruise",
        "sicario",
        "American gangster",
        "Jaws",
        "intern",
        "captin phillips",
        "The highwaymen",
        "Eat pray love",
        "the two popes",
        "the irishmen",
        "Jaws",
    ];

    let _apple_tv = vec!["Trainspotting"];

    let amazon_movies = vec![
        "Baby drive",
        "Pulp fiction",
        "Django unchained",
        "Crash",
        "no country for old men",
        "Grand budapest hotel",
        "Taxi Driver",
        "Moon light",
        "Juno",
        "Poor things",
        "American Beauty",
        "Dr Strange Love",
        "Airplane",
        "The social network",
        "The Phoenician",
        "The accountant",
        "The accountant 2",
        "the revenant",
        "Challengers",
        "Black klansman",
        "The death of stalin",
        "Birdman",
        "Licorice pizza",
    ];

    let youtube_movies = vec![
        "Asteroid city",
        "No country for old men",
        "Dallas buyers club",
        "Blue velvet",
        "The big short",
        "Margin call",
        "vice",
    ];

    let mut movies: Vec<String> = Vec::new();

    for movie in hulu_movies {
        movies.push("Hulu ".to_owned() + movie);
    }

    for movie in netflix_movies {
        movies.push("Netflix ".to_owned() + movie);
    }

    for movie in amazon_movies {
        movies.push("Amazon ".to_owned() + movie);
    }

    for movie in youtube_movies {
        movies.push("Youtube ".to_owned() + movie);
    }

    vec![movies.choose(&mut rand::rng()).unwrap().to_string()]
}

pub fn makeup() {
    let lip_options = ["Kush Lip Oil", "Summer Fridays", "Peach Glaze"];

    let scents = ["Glossier scent", "Britney Spears scent"];

    println!("Mascara");
    println!("Eye Liner");
    let lip_option = lip_options.choose(&mut rand::rng()).unwrap();
    println!("lip: {lip_option}");
    let scent = scents.choose(&mut rand::rng()).unwrap();
    println!("{scent}");
}

pub fn get_current_week() -> u32 {
    let today = Local::now().date_naive();
    today.iso_week().week()
}
