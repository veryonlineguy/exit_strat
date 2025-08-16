use crate::util;
use polyfit_rs::polyfit_rs::polyfit;
use rusqlite::{Connection, Result};
use std::collections::HashMap;

pub fn caffeine_summary() -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    let mut stmt = conn.prepare(
        "select sum(caffeine_mg), date(date_time) from caffeine group by date(date_time) limit 7",
    )?;

    let caff_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, u32>(0)?, row.get::<_, String>(1)?))
    })?;

    for caff in caff_iter {
        let (amount, date) = caff?;
        println!("{date}: {amount}");
    }

    Ok(())
}

fn query_work_table(name: String) -> Result<u32> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    let stmt = "select sum(time) from ".to_owned()
        + &name
        + " where date(date_time) == date(datetime('now','localtime'));";

    let Ok(mut stmt) = conn.prepare(&(stmt)) else {
        panic!("WTF");
    };

    if let Ok(iter) = stmt.query_map([], |row| Ok(row.get::<_, u32>(0))) {
        for res in iter {
            if let Ok(res) = res {
                if let Ok(res) = res {
                    return Ok(res);
                }
            }
        }
    }

    Ok(0)
}

fn query_reading() -> Result<Vec<(String, u32)>> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    let mut stmt = conn.prepare(
       &("select type, sum(time) from reading where date(date_time) == date(datetime('now','localtime')) group by type;"),
    )?;

    let iter = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, u32>(1)?))
    })?;

    let mut ret = Vec::new();
    for res in iter {
        ret.push(res?);
    }

    Ok(ret)
}
pub fn work_summary() -> Result<()> {
    let current_tasks = ["link", "pwn_college", "ccna", "exit"];

    println!("# Productivity stats:  ");
    let mapping = HashMap::from([
        ("link".to_string(), "Reading security articles".to_string()),
        (
            "pwn_college".to_string(),
            "Studying on pwn.college".to_string(),
        ),
        ("exit".to_string(), "Working on todo app".to_string()),
        ("pleasure".to_string(), "Reading fun book".to_string()),
        ("technical".to_string(), "Reading the rust book".to_string()),
    ]);

    let mut res = Vec::new();
    for task in current_tasks {
        let time = query_work_table(task.to_string())?;
        res.push((task.to_string(), time));
    }

    for task in query_reading()? {
        res.push(task);
    }

    fn format_time(mins: u32) -> String {
        if mins < 60 {
            format!("{: <2} mins", mins)
        } else {
            let hours = mins / 60;
            let mins = mins % 60;
            format!("{hours} hours {mins} mins")
        }
    }

    for (task, time) in &res {
        let pretty = mapping.get(task).unwrap();
        let time = format_time(*time);
        println!("- {: <25} for {time}  ", pretty);
    }

    Ok(())
}

pub fn spend_summary() -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    let mut stmt = conn.prepare(
        "select date(date_time), sum(amount) from spend group by date(date_time) order by date(date_time) asc limit 7;",
    )?;

    let spend_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
    })?;

    for spend in spend_iter {
        let (date, amount) = spend?;
        let amount = format!("{:.2}", amount);
        let amount = "$".to_owned() + &amount;
        println!("{date}: {amount: >7}  ");
    }

    Ok(())
}

fn fit_rowing() -> Result<()> {
    let db_path = util::get_database_path()?;
    let conn = Connection::open(&db_path)?;

    let mut stmt =
        conn.prepare("select distance, time from row  order by date_time asc limit 30;")?;

    let spend_iter =
        stmt.query_map([], |row| Ok((row.get::<_, f64>(0)?, row.get::<_, f64>(1)?)))?;

    let mut distances = vec![];
    let mut times = vec![];
    for spend in spend_iter {
        let (distance, time) = spend?;
        distances.push(distance);
        times.push(time);
    }

    let [a, b, c] = polyfit(&distances, &times, 2).unwrap().try_into().unwrap();

    let x: f64 = 2000.0;

    fn conv_time(seconds: u64) -> String {
        let minutes = seconds / 60;
        let second = seconds % 60;

        format!("{}:{}", minutes, second).to_string()
    }

    let twoktime = a + b * x + c * x.powf(2.0);

    let split = twoktime / 4.0;

    let weight_lb = 280.0;
    let weight_kg = weight_lb * 0.453592;

    let sec = (twoktime as u64) % 60;
    let sec = (sec as f64) / 60.0;

    let min = (twoktime as u64 / 60);

    let mins = min as f64 + sec;

    let y = 15.7 - (1.5 * mins);

    let vo2 = (y * 1000.0) / weight_kg;

    println!(
        "2k time pred {} split {}  ",
        conv_time(twoktime as u64),
        conv_time(split as u64)
    );
    println!("Vo2 Estimate {:.1}  ", vo2);
    Ok(())
}

pub fn report() -> Result<()> {
    spend_summary()?;
    fit_rowing()?;

    Ok(())
}
