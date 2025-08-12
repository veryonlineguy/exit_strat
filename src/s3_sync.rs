use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, primitives::ByteStream};
use chrono::{Datelike, Duration, Local, NaiveDate};
use std::fs;
use tokio::runtime::Runtime;

pub fn sync_data_to_s3() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(sync_data_to_s3_async())
}

async fn sync_data_to_s3_async() -> Result<(), Box<dyn std::error::Error>> {
    // Load AWS configuration
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&config);

    let bucket_name = "veryonlineguy";
    let data_dir = "data";

    // Create folder name with current date (MM-DD-YYYY-data format)
    let today = Local::now();
    let folder_name = format!(
        "{:02}-{:02}-{}-data",
        today.month(),
        today.day(),
        today.year()
    );

    // Read all files in the data directory
    let entries = fs::read_dir(data_dir)?;

    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() {
            let file_name = file_path
                .file_name()
                .ok_or("Invalid file name")?
                .to_str()
                .ok_or("Invalid UTF-8 in file name")?;

            // Read file contents
            let file_contents = fs::read(&file_path)?;
            let body = ByteStream::from(file_contents);

            // Create S3 key with folder structure
            let s3_key = format!("{}/{}", folder_name, file_name);

            // Upload to S3
            client
                .put_object()
                .bucket(bucket_name)
                .key(&s3_key)
                .body(body)
                .send()
                .await?;
        }
    }

    cleanup_old_backups(&client, bucket_name).await?;

    Ok(())
}

async fn cleanup_old_backups(
    client: &Client,
    bucket_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Calculate the cutoff date (3 weeks ago)
    let cutoff_date = Local::now().date_naive() - Duration::weeks(3);

    // List all objects in the bucket
    let resp = client.list_objects_v2().bucket(bucket_name).send().await?;

    let objects = resp.contents();
    for object in objects {
        if let Some(key) = object.key() {
            // Parse the folder name to extract the date
            if let Some(folder_date) = parse_folder_date(key) {
                if folder_date < cutoff_date {
                    // Delete the object
                    client
                        .delete_object()
                        .bucket(bucket_name)
                        .key(key)
                        .send()
                        .await?;
                }
            }
        }
    }

    Ok(())
}

fn parse_folder_date(key: &str) -> Option<NaiveDate> {
    // Extract folder name from key (e.g., "07-24-2025-data/file.csv" -> "07-24-2025-data")
    let folder_name = key.split('/').next()?;

    // Check if it matches our backup folder pattern (MM-DD-YYYY-data)
    if !folder_name.ends_with("-data") {
        return None;
    }

    // Remove the "-data" suffix
    let date_part = folder_name.strip_suffix("-data")?;

    // Split by '-' to get [MM, DD, YYYY]
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.len() != 3 {
        return None;
    }

    // Parse the date components
    let month: u32 = parts[0].parse().ok()?;
    let day: u32 = parts[1].parse().ok()?;
    let year: i32 = parts[2].parse().ok()?;

    // Create NaiveDate
    NaiveDate::from_ymd_opt(year, month, day)
}
