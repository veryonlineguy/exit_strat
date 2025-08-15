mod affirmations;
mod lift;
mod logging;
mod menu;
mod model1;
mod publish;
mod s3_sync;
mod schedule;
mod stats;
mod util;

use chrono::Duration;
use chrono::{Datelike, Local, NaiveDate};
use clap::Parser;
use csv::WriterBuilder;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;

use crate::util::VAULT_DIR;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(long)]
    week_no: bool,

    /// Print Monday's schedule to stdout
    #[arg(long)]
    monday: bool,

    #[arg(long)]
    report: bool,

    #[arg(long)]
    log_time: bool,
    /// Print Tuesday's schedule to stdout
    #[arg(long)]
    tuesday: bool,

    #[arg(long)]
    publish: bool,

    /// Print Wednesday's schedule to stdout
    #[arg(long)]
    wednesday: bool,

    /// Print Thursday's schedule to stdout
    #[arg(long)]
    thursday: bool,

    /// Print Friday's schedule to stdout
    #[arg(long)]
    friday: bool,

    /// Print Saturday's schedule to stdout
    #[arg(long)]
    saturday: bool,

    /// Print Sunday's schedule to stdout
    #[arg(long)]
    sunday: bool,

    #[arg(long)]
    weight: bool,

    #[arg(long)]
    predict: bool,

    #[arg(long)]
    lift: bool,

    #[arg(long)]
    model1: bool,

    #[arg(long)]
    model2: bool,

    #[arg(long)]
    month: bool,

    #[arg(long)]
    financial: bool,

    #[arg(long)]
    bored: bool,

    #[arg(long)]
    debt: bool,

    #[arg(long)]
    workout: bool,

    #[arg(long)]
    violation: bool,

    #[arg(long)]
    win: bool,

    #[arg(long)]
    log: bool,

    #[arg(long)]
    sync: bool,

    #[arg(long)]
    makeup: bool,

    #[arg(long)]
    read: bool,

    #[arg(long)]
    query_undone: bool,

    #[arg(long)]
    task_summary: bool,

    #[arg(long)]
    next: bool,

    #[arg(long)]
    exit: bool,

    #[arg(long)]
    work_sum: bool,

    #[arg(long)]
    spend: bool,

    #[arg(long)]
    row: bool,
}

// Simple number reader
fn read_input<T: std::str::FromStr>() -> T {
    loop {
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<T>() {
            Ok(val) => return val,
            Err(_) => {
                println!("Invalid input, try again:");
            }
        }
    }
}

fn log_weight() {
    let weight: f64;
    let cals_realized: u32;

    print!("Enter today's am weight (e.g. 294.6): ");
    weight = read_input();

    print!("Enter cals_realized (e.g. 1979): ");
    cals_realized = read_input();

    print!("Enter protein in g ");
    let protein: u32 = read_input();

    print!("Enter strava RE score ");
    let strava_re: u32 = read_input();

    let today: String = Local::now().format("%Y-%m-%d").to_string();

    let record = vec![
        today,
        format!("{weight}"),
        format!("{}", cals_realized as f64),
        format!("{}", strava_re),
        format!("{}", protein),
    ];
    println!();
    println!("Update calendar");

    let file_path = "data/weight_energy.csv";
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open weight.csv");

    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);

    wtr.write_record(&record).expect("Write failed");
    wtr.flush().expect("Flush failed");
}

fn month_gen() {
    let today = Local::now();
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "Agust",
        "September",
        "October",
        "November",
        "December",
    ];

    for (idx, month) in months.into_iter().enumerate() {
        println!("{month}: {}", idx + 1);
    }
    print!("Month to gen: ");
    let pick: u32 = read_input();

    //let (year, month) = (today.year(), today.month());
    let (year, month) = (today.year(), pick);
    let mut day = 1;

    loop {
        if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
            let weekday_letter = date
                .weekday()
                .to_string()
                .to_lowercase()
                .chars()
                .nth(0)
                .unwrap();
            println!("{}{} ", day, weekday_letter);
            day += 1;
        } else {
            break;
        }
    }
}

fn main() {
    let args = Args::parse();
    if args.weight {
        log_weight();

        // Sync data directory to S3
        println!("Syncing data to S3...");
        if let Err(e) = s3_sync::sync_data_to_s3() {
            eprintln!("Error syncing data to S3: {}", e);
        }

        schedule::write_tomorrow_to_vault();
        model1::run("data/weight_energy.csv").unwrap();

    }

    if args.publish {
        publish::publish();
    }

    if args.sync {
        println!("Syncing data to S3...");
        if let Err(e) = s3_sync::sync_data_to_s3() {
            eprintln!("Error syncing data to S3: {}", e);
        }
    }

    if args.predict {
        model1::run("data/weight_energy.csv").unwrap();
    }

    if args.model1 {
        model1::run("data/weight_energy.csv").unwrap();
    }

    if args.model2 {
        model1::run_one_state("data/weight_energy.csv").unwrap();
    }

    fn read_task(name: String) -> i32 {
        print!("Enter {} amount in mins: ", name);
        read_input()
    }

    if args.read {
        logging::log_reading(
            read_task("Reading Pleasure".to_string()),
            String::from("pleasure"),
        )
        .unwrap();
    }

    if args.report {
        stats::report().unwrap();
    }

    if args.bored {
        print!("Start stopwatch Y/n: ");
        let mut input: String = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        // Link + Read pleasure
        if rand::random_bool(0.3) {
            if rand::random_bool(0.5) {
                logging::log_reading(
                    read_task("Reading technical".to_string()),
                    String::from("technical"),
                )
                .unwrap();
            } else {
                logging::log_link(read_task("Link".to_string())).unwrap();
            }
        } else {
            if rand::random_bool(0.6) {
                logging::log_ccna(read_task("CCNA".to_string())).unwrap();
            } else {
                logging::log_pwn_college(read_task("Pwn.college".to_string())).unwrap();
            }
        }

        print!("Updated Anki Y/n: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
    }

    if args.month {
        month_gen();
    }

    if args.next {
        schedule::utils::write_tomorrow_to_vault();
    }

    if args.week_no {
        println!("{}", crate::schedule::utils::current_week_number());
    }

    if args.lift {
        println!("{}", lift::get_lifts());
    }

    if args.violation {
        if let Err(e) = logging::log_violations() {
            eprintln!("Error logging violation: {}", e);
        }
    }

    if args.spend {
        if let Err(e) = logging::log_spend() {
            eprintln!("Error logging violatio: {}", e);
        }
    }

    if args.win {
        if let Err(e) = logging::log_win() {
            eprintln!("Error logging win: {}", e);
        }
    }

    if args.exit {
        print!("Enter exit amount in mins: ");
        let time: i32 = read_input();
        if let Err(e) = logging::log_exit(time) {
            eprintln!("Error logging exit intake: {}", e);
        }
    }

    if args.log_time {
        logging::log_time().unwrap();
    }

    if args.makeup {
        schedule::utils::makeup();
    }

    if args.monday {
        schedule::print_monday_schedule();
    }

    if args.debt {
        logging::log_debt().unwrap();
    }
    if args.tuesday {
        schedule::print_tuesday_schedule();
    }

    if args.wednesday {
        schedule::print_wednesday_schedule();
    }

    if args.log {
        logging::log_tasks();
    }

    if args.row {
        logging::log_row().unwrap();
    }

    if args.work_sum {
        stats::work_summary().unwrap();
    }

    if args.thursday {
        schedule::print_thursday_schedule();
    }

    if args.friday {
        schedule::print_friday_schedule();
    }

    if args.saturday {
        schedule::print_saturday_schedule();
    }

    if args.sunday {
        schedule::print_sunday_schedule();
    }
}
