use crate::VAULT_DIR;
use crate::read_input;
use crate::util;
use chrono::{DateTime, Datelike, Local, NaiveDate};
use regex::Regex;
use rusqlite::{Connection, Result, params};
use std::fs;
use std::io::{self, Write};

#[derive(Clone, Debug)]
struct Task {
    id: u8,
    name: String,
}

pub fn log_reading(time: i32, rtype: String) -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS reading (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date_time TEXT NOT NULL,
            time INTEGER NOT NULL,
            type TEXT NOT NULL
        )",
        [],
    )?;

    let now: DateTime<Local> = Local::now();
    let date_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO reading (date_time, time,type) VALUES (?1, ?2, ?3)",
        [&date_time_str, &time.to_string(), &rtype],
    )?;

    println!("Logged reading: {} min at {}", time, date_time_str);

    Ok(())
}

pub fn log_time() -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS time (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date_time TEXT NOT NULL,
            category TEXT NOT NULL,
            time INTEGER NOT NULL
        )",
        [],
    )?;

    let mut stmt = conn.prepare("select category from time group by category;")?;

    let past_violations = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?))?;

    let mut options = Vec::new();

    for (idx, violation) in past_violations.enumerate() {
        let name = violation?;
        options.push(name.clone());
        println!("{idx}: {name}");
    }

    print!("Enter Category: ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let choice = match input.trim().parse::<usize>() {
        Ok(val) => options[val].clone(),
        Err(_) => input.trim().to_string(),
    };

    print!("Enter Time (minutes): ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let time = match input.trim().parse::<u64>() {
        Ok(val) => val,
        Err(_) => panic!("Couldn't parse amount"),
    };

    conn.execute(
        "INSERT INTO time (date_time, category, time) VALUES (datetime('now', 'localtime'), ?1, ?2)",
        (&choice, &time),
    )?;

    println!("Logged violation: {}", choice);

    Ok(())
}

pub fn log_violations() -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS violations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date_time TEXT NOT NULL,
            name TEXT NOT NULL
        )",
        [],
    )?;

    let mut stmt = conn.prepare("select name from violations group by  name;")?;

    let past_violations = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?))?;

    let mut options = Vec::new();

    for (idx, violation) in past_violations.enumerate() {
        let name = violation?;
        options.push(name.clone());
        println!("{idx}: {name}");
    }

    print!("Enter violation: ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let choice = match input.trim().parse::<usize>() {
        Ok(val) => options[val].clone(),
        Err(_) => input.trim().to_string(),
    };
    conn.execute(
        "INSERT INTO violations (date_time, name) VALUES (datetime('now', 'localtime'), ?1)",
        [&choice],
    )?;

    println!("Logged violation: {}", choice);

    Ok(())
}

pub fn log_win() -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS win (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date_time TEXT NOT NULL,
            name TEXT NOT NULL
        )",
        [],
    )?;

    let mut stmt = conn.prepare("select name from win group by  name;")?;

    let past_win = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?))?;

    let mut options = Vec::new();

    for (idx, violation) in past_win.enumerate() {
        let name = violation?;
        options.push(name.clone());
        println!("{idx}: {name}");
    }

    print!("Enter violation: ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let choice = match input.trim().parse::<usize>() {
        Ok(val) => options[val].clone(),
        Err(_) => input.trim().to_string(),
    };
    conn.execute(
        "INSERT INTO win (date_time, name) VALUES (datetime('now', 'localtime'), ?1)",
        [&choice],
    )?;

    println!("Logged win: {}", choice);

    Ok(())
}

pub fn _log_big_game_hunting(time: i32) -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS big_game_hunting (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date_time TEXT NOT NULL,
            time INTEGER NOT NULL
        )",
        [],
    )?;

    let now: DateTime<Local> = Local::now();
    let date_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO big_game_hunting (date_time, time) VALUES (?1, ?2)",
        [&date_time_str, &time.to_string()],
    )?;

    println!("Logged Big Gme Hunting: {} min at {}", time, date_time_str);

    Ok(())
}

pub fn log_link(time: i32) -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS link (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date_time TEXT NOT NULL,
            time INTEGER NOT NULL
        )",
        [],
    )?;

    let now: DateTime<Local> = Local::now();
    let date_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO link (date_time, time) VALUES (?1, ?2)",
        [&date_time_str, &time.to_string()],
    )?;

    println!("Logged link: {} min at {}", time, date_time_str);

    Ok(())
}

pub fn log_debt() -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS debt (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date_time TEXT NOT NULL,
            bofa_travel REAL NOT NULL,
            bofa_reward REAL NOT NULL,
            discover REAL NOT NULL,
            imprint REAL NOT NULL,
            paypal REAL NOT NULL,
            affirm REAL NOT NULL,
            cisco REAL NOT NULL,
            medical REAL NOT NULL,
            schwab REAL NOT NULL
            )",
        [],
    )?;

    print!("Enter bofa_travel: ");
    let bofa_travel: f64 = read_input();

    print!("Enter bofa_reward: ");
    let bofa_reward: f64 = read_input();

    print!("Enter discover: ");
    let discover: f64 = read_input();

    print!("Enter imprint: ");
    let imprint: f64 = read_input();
    print!("Enter paypal: ");
    let paypal: f64 = read_input();
    print!("Enter affirm: ");
    let affirm: f64 = read_input();
    print!("Enter cisco: ");
    let cisco: f64 = read_input();

    print!("Enter medical: ");
    let medical: f64 = read_input();

    print!("Enter schwab: ");
    let schwab: f64 = read_input();

    conn.execute(
        "INSERT INTO debt (date_time, bofa_travel, bofa_reward, discover, imprint, paypal, affirm, cisco, medical, schwab) VALUES (datetime('now', 'localtime'), ?1, ?2, ?3,?4,?5,?6,?7,?8,?9)",
        (bofa_travel, bofa_reward, discover, imprint, paypal, affirm, cisco, medical, schwab),
    )?;

    Ok(())
}

pub fn log_ccna(time: i32) -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ccna (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date_time TEXT NOT NULL,
            time INTEGER NOT NULL
        )",
        [],
    )?;

    let now: DateTime<Local> = Local::now();
    let date_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO ccna (date_time, time) VALUES (?1, ?2)",
        [&date_time_str, &time.to_string()],
    )?;

    println!("Logged ccna: {} min at {}", time, date_time_str);

    Ok(())
}

pub fn log_pwn_college(time: i32) -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS pwn_college (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date_time TEXT NOT NULL,
            time INTEGER NOT NULL
        )",
        [],
    )?;

    let now: DateTime<Local> = Local::now();
    let date_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO pwn_college (date_time, time) VALUES (?1, ?2)",
        [&date_time_str, &time.to_string()],
    )?;

    println!("Logged pwn.college: {} min at {}", time, date_time_str);

    Ok(())
}

pub fn log_spend() -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS spend (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date_time TEXT NOT NULL,
            amount REAL NOT NULL,
            name TEXT NOT NULL,
            category TEXT NOT NULL
        )",
        [],
    )?;

    let mut stmt = conn.prepare("select category from spend group by category;")?;

    let past_categories = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?))?;

    let mut options = Vec::new();

    for (idx, violation) in past_categories.enumerate() {
        let name = violation?;
        options.push(name.clone());
        println!("{idx}: {name}");
    }

    print!("Enter Category: ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let category = match input.trim().parse::<usize>() {
        Ok(val) => options[val].clone(),
        Err(_) => input.trim().to_string(),
    };

    print!("Enter Name: ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let name = input.trim();

    print!("Enter Amount: ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let amount = match input.trim().parse::<f64>() {
        Ok(val) => val,
        Err(_) => panic!("Couldn't parse amount"),
    };
    conn.execute(
        "INSERT INTO spend (date_time, amount, name, category) VALUES (datetime('now', 'localtime'), ?1, ?2, ?3)",
        (&amount, &name, &category),
    )?;

    println!("Logged spend: {} {}: {}", name, amount, category);

    Ok(())
}

pub fn log_exit(time: i32) -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS exit (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date_time TEXT NOT NULL,
            time INTEGER NOT NULL
        )",
        [],
    )?;

    let now: DateTime<Local> = Local::now();
    let date_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO exit (date_time, time) VALUES (?1, ?2)",
        [&date_time_str, &time.to_string()],
    )?;

    println!("Logged exit: {} min at {}", time, date_time_str);

    Ok(())
}

pub fn log_caffeine(caffeine_mg: i32) -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS caffeine (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date_time TEXT NOT NULL,
            caffeine_mg INTEGER NOT NULL
        )",
        [],
    )?;

    let now: DateTime<Local> = Local::now();
    let date_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO caffeine (date_time, caffeine_mg) VALUES (?1, ?2)",
        [&date_time_str, &caffeine_mg.to_string()],
    )?;

    println!(
        "Logged caffeine intake: {}mg at {}",
        caffeine_mg, date_time_str
    );

    Ok(())
}

pub fn log_row() -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS row (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date_time TEXT NOT NULL,
            distance INTEGER NOT NULL,
            time INTEGER NOT NULL,
            watts INTEGER NOT NULL,
            cals INTEGER NOT NULL
        )",
        [],
    )?;

    let now: DateTime<Local> = Local::now();
    let date_time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    print!("Enter distance (m): ");
    let distance: u64 = read_input();

    print!("Enter time (m): ");
    let time: u64 = read_input();

    print!("Enter watts: ");
    let watts: f64 = read_input();

    print!("Enter cals: ");
    let cals: f64 = read_input();

    conn.execute(
        "INSERT INTO row (date_time, distance, time, watts, cals) VALUES (datetime('now', 'localtime'), ?1, ?2, ?3, ?4 )",
        (distance, time, watts, cals),
    )?;

    println!("Logged {}m row", distance);

    Ok(())
}

fn tasks_to_insert(conn: &Connection, tasks: Vec<Task>) -> Vec<Task> {
    let mut stmt = conn.prepare(
        "select task_id from completed_todos where date(date_time) ==  date(datetime('now', 'localtime'))",
    ).unwrap();

    let id_iter = stmt
        .query_map([], |row| Ok(row.get::<_, u8>(0).unwrap()))
        .unwrap();

    let inserted_id: Vec<u8> = id_iter.into_iter().map(|x| x.unwrap()).collect();

    let mut to_insert = Vec::new();
    for task in &tasks {
        let id = task.id;
        if !inserted_id.contains(&id) {
            to_insert.push(task.clone());
        }
    }

    to_insert
}

fn insert_task(conn: &Connection, task: &Task) {
    conn.execute(
        "INSERT INTO completed_todos (task_id, date_time, task) VALUES (?1,datetime('now','localtime'), ?2)",
        params![task.id, task.name],
    ).unwrap();
}
pub fn log_tasks() {
    let db_path = util::get_database_path().unwrap();
    let conn = Connection::open(&db_path).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS completed_todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id INTEGER NOT NULL,
            date_time TEXT NOT NULL,
            task TEXT NOT NULL
        );",
        [],
    )
    .unwrap();

    let local: DateTime<Local> = Local::now();
    let today: NaiveDate =
        NaiveDate::from_ymd_opt(local.year(), local.month(), local.day()).unwrap();
    let date_str = today.format("%Y-%m-%d").to_string();
    let filename = format!("{}/{}.md", VAULT_DIR, date_str);

    let re = Regex::new(r"([0-9]+). \[x\] ([0-9a-zA-z ]+)").unwrap();

    let buff: String = fs::read_to_string(filename).unwrap();

    let mut tasks = Vec::new();
    for (_, [id, name]) in re.captures_iter(&buff).map(|c| c.extract()) {
        tasks.push(Task {
            id: id.parse().expect("Couldn't parse id"),
            name: name.to_string(),
        })
    }

    let tasks = tasks_to_insert(&conn, tasks);

    for task in &tasks {
        insert_task(&conn, task);
    }
}
