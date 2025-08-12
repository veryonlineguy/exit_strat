use crate::VAULT_DIR;
use chrono::{DateTime, Datelike, Days, Local, NaiveDate};
use std::fs;
use std::process::Command;
use std::process::Output;
use std::time::{SystemTime, UNIX_EPOCH};

fn handle_out(out: Output) {
    println!("{}", str::from_utf8(&out.stdout).unwrap());
    if out.stderr.len() != 0 {
        println!("len {}", out.stderr.len());
        println!("stderr: {}", str::from_utf8(&out.stderr).unwrap());
        // exit(69);
    }
}

pub fn publish() {
    let paths = fs::read_dir("data/vault/Weekly Reflection").unwrap();

    let bucket_name = "veryonlineguy.com";
    let distribution_id = "E1FCC9GGT96QM6";

    for path in paths {
        let path = path.unwrap().path().display().to_string();
        let today_str = fs::read_to_string(&path).unwrap();

        let file_name = path.split("/").last().unwrap();

        let name = &file_name[0..8];

        let year: i32 = name[0..4].parse().unwrap();
        let week: u32 = name[6..8].parse().unwrap();

        let sunday = NaiveDate::from_yo_opt(year, 7 * week - 9).unwrap();
        let date = sunday.format("%Y-%m-%d").to_string();
        let header = "+++\ntitle = \"".to_owned()
            + name
            + "\"\ndate = \""
            + &&date
            + "\"\ndescription = \"\"\ntags = []\n+++\n";

        let write_path = "veryonlineguy.com/content/".to_string() + &file_name;

        let today_str = header + &today_str;
        fs::write(write_path, &today_str).unwrap();
    }

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        .to_string();

    let invalidation_cmd = format!(
        r#"Paths={{Quantity=1,Items=["/*"]}},CallerReference="{}""#,
        since_the_epoch
    );
    let out = Command::new("hugo")
        .current_dir("veryonlineguy.com")
        .arg("build")
        .output()
        .expect("npm build failed");

    handle_out(out);

    let out = Command::new("aws")
        .arg("s3")
        .arg("rm")
        .arg("s3://".to_owned() + &bucket_name)
        .arg("--recursive")
        .output()
        .expect("Couldn't clean bucket");
    handle_out(out);

    let out = Command::new("aws")
        .arg("s3")
        .arg("sync")
        .arg("veryonlineguy.com/public/")
        .arg("s3://".to_owned() + &bucket_name)
        .output()
        .expect("Couldn't upload to bucket");

    handle_out(out);

    let invalidation_id = Command::new("aws")
        .arg("cloudfront")
        .arg("create-invalidation")
        .arg("--distribution-id")
        .arg(&distribution_id)
        .arg("--invalidation-batch")
        .arg(invalidation_cmd)
        .arg("--query")
        .arg("Invalidation.Id")
        .arg("--output")
        .arg("text")
        .output()
        .expect("Couldn't create invalidation");

    handle_out(invalidation_id.clone());

    let invalidation_id = str::from_utf8(&invalidation_id.stdout).unwrap().trim();
    let wait_out = Command::new("aws")
        .arg("cloudfront")
        .arg("wait")
        .arg("invalidation-completed")
        .arg("--distribution-id")
        .arg(&distribution_id)
        .arg("--id")
        .arg(invalidation_id)
        .output()
        .expect("Couldn't wait on invalidation");

    handle_out(wait_out);
}
